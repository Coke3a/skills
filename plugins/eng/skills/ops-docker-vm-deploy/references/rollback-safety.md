# Rollback safety

Most deploy scripts have a rollback block that has never run. This file is about making it
actually run, and actually work, when it is needed.

## `set -e` will skip the rollback you wrote

The standard opener is `set -euo pipefail`, and it is correct — an unnoticed failure
halfway through a deploy is worse than a loud stop. But it interacts badly with a rollback
block placed at the bottom of the script:

```bash
set -euo pipefail
printf 'TAG=%s\n' "$NEW" > .env     # host state mutated
docker compose up -d                # ← fails
...
if [ "$healthy" != 1 ]; then        # ← never reached; set -e already exited
  printf 'TAG=%s\n' "$OLD" > .env
fi
```

The run goes red, which is good. But the host is left with `.env` pointing at a tag that
does not work, and the rollback is skipped **exactly in the cases where the failure was
most severe** — the container could not be created at all. The block only ever protects
you against the milder failure where the container starts but reports unhealthy.

The fix is to capture the outcome instead of letting it propagate:

```bash
ok=1
docker compose up -d || ok=0

if [ "$ok" = 1 ] && wait_healthy; then
  ... success ...
  exit 0
fi
... rollback ...
exit 1
```

Apply this to **every** command between the first mutation and the success branch. In
practice that is the registry pull and the container start. Commands that only read, or
that already end in `|| true`, are fine.

## Order operations so the risky-but-independent work happens first

Better than capturing every failure is having fewer failures to capture. Registry login can
fail (expired token, network); it does not depend on any file you are about to change. Move
it above the first mutation and it needs no special handling at all — if it fails, the host
is untouched and `set -e` exiting is exactly right.

The general shape:

```
1. preflight checks         ← nothing mutated yet, plain `exit 1` is fine
2. registry login           ← can fail freely
3. install/validate config  ← first mutation; validation failure restores and exits
4. pull image               ← captured
5. write tag, start         ← captured
6. health check → success or rollback
```

## Every mutation needs a paired restore

If a deploy changes both the image tag and a config file, a rollback that restores only the
tag produces **old image + new config** — a combination that has never run anywhere. That
is not the previous state; it is a third state, untested, reached only during an incident.

```bash
# before installing the new config
if [ -f config.yml ]; then cp -f config.yml config.yml.prev; fi

# in the rollback path
if [ -f config.yml.prev ]; then mv -f config.yml.prev config.yml; fi
```

Restoring is the easy half. The discipline is noticing *every* mutation: the tag file, the
config file, anything the script writes. Enumerate them at the top of the rollback block.

## A rollback that is not verified is a log that lies

```bash
printf 'TAG=%s\n' "$OLD" > .env
docker compose up -d
echo "rolled back to $OLD"          # printed unconditionally
exit 1
```

This prints success even when the old container also fails to start. Someone reads
"rolled back", believes the site is up, and goes to bed while it is down.

Extract the health poll into a function and call it twice — once after deploy, once after
rollback — and make the two outcomes say different things:

```bash
if docker compose up -d && wait_healthy; then
  echo "rolled back to $OLD — healthy"
else
  echo "ROLLBACK FAILED — the service is likely down, intervene now"
fi
exit 1
```

The run exits non-zero either way. The difference is whether the human reading the log at
2am knows they have to get up.

## `[ -f x ] && cmd` is itself a `set -e` trap

At the top level of a script under `set -e`:

```bash
[ -f foo ] && mv foo bar     # if foo does not exist, this line exits the script
```

The `&&` list returns the test's non-zero status, and unlike a bare failing command inside
an `if` condition, nothing here is exempt. Since "the file might not exist" is the whole
reason you wrote the test, this fails precisely in the case you were guarding against.
Use `if ... fi` in scripts that must not die unexpectedly.

## Preflight: fail with the real cause

Check the host's prerequisites before touching anything, and name them:

```bash
if [ ! -f app.env ]; then
  echo "app.env is missing on the host — see SETUP.md"
  exit 1
fi
```

Without this, a missing env file surfaces later as a compose validation error, and the
script reports "config file is invalid" — sending the next person to debug a file that is
perfectly fine. Cheap checks that produce accurate messages are worth more than they look.

## The rollback window is finite

Pruning old images keeps the disk from filling, but it also bounds how far back you can go.
`--filter until=168h` means seven days. Say so in the runbook, because the manual recovery
path (`docker pull <old-tag>`) needs registry credentials that the automated path never
required. That is the one place a long-lived token is still needed — store it in a password
manager, not on the host.
