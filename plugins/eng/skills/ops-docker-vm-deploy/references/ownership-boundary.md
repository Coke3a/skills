# Ownership boundary

The single most useful artefact in this kind of deploy is a table saying, for every file
the host needs, who is allowed to write it. Draw it before writing any YAML.

## The rule

**One writer per file.**

- In git and the host needs it → **CI ships it on every deploy.**
- Holds secrets, or is specific to this one machine → **lives only on the host, CI never
  touches it, and it is never committed.**

A file with two writers eventually diverges, and divergence in this direction is silent:
you edit it in git, review it, merge it, CI goes green, and nothing changes on the server.
Nobody finds out until someone SSHes in for an unrelated reason and notices the file does
not match the repo.

## A worked example

```
/opt/<app>/
├── docker-compose.yml   ← CI ships every deploy. Edit in git, never on the box.
├── .env                 ← CI overwrites every deploy. Holds TAG= and nothing else.
└── app.env              ← Host only. Typed by hand once, chmod 600, never in git.
```

The split between `.env` and `app.env` is the part people collapse, and it is load-bearing.
Compose reads them for different purposes:

| | Read when | Purpose |
|---|---|---|
| `.env` | Compose parses the YAML | Interpolates `${TAG}` **into the compose file** |
| `app.env` | Container starts | Becomes environment variables **inside the container** |

Because CI overwrites `.env` on every deploy, any secret placed there disappears on the
next push. That is not a bug to work around — it is the property that makes it safe for CI
to own that file. Keep secrets in the file CI cannot touch.

## Things that look like they belong in git but do not

- **Reverse proxy config.** It lives under `/etc/nginx/`, needs root, needs `nginx -t`
  before reload, and changes maybe twice in the life of the service. Keep a copy in git as
  documentation; install it by hand. Automating a two-time change buys nothing and adds a
  way to take down every other site on the box.
- **TLS certificates and ACME state.** Machine-specific, renewed by a timer.
- **Anything in `authorized_keys`.** Adding a key is a deliberate act with an audit trail.

## Do not `git pull` on the production host

It looks simpler than shipping files from CI. It is not:

1. It requires git credentials on the machine that serves traffic. Trust currently runs one
   way (CI → server); a deploy key turns it into two ways, so compromising the box also
   yields read access to the repo.
2. It puts source and `.git` back on a host whose whole design is that it has neither.
   Build tooling that inspects git (`git rev-parse`, `git branch`) starts behaving
   differently in production than in the image.
3. A local edit someone made months ago turns into a merge conflict, `git pull` exits
   non-zero, and under `set -e` the deploy dies at an unpredictable point.

Shipping one file from the runner has none of these properties, and the runner already has
a checkout of exactly the commit that produced the image — so the config and the image can
never be from different commits.

## How CI should ship a file

Stage it, then install it inside the deploy script where ordering is under your control:

```
scp  →  /opt/<app>/incoming/<file>
script:
  backup current → install from incoming/ → validate → (restore on failure)
```

Staging matters because the SSH step runs *after* the copy step. If the copy writes
straight to the live path, then by the time your script runs its "back up the current
file" line, the current file is already the new one, and the backup is worthless.

Two hard constraints:

- **Name the file explicitly.** Never sync the directory. A `--delete` sync, or a tool
  flag that clears the target first, removes `app.env` — the one file with no other copy
  anywhere in the world.
- **Validate before mutating anything else.** If the shipped file is malformed, restore the
  previous one and exit before touching containers, so a bad commit costs you a red build
  instead of an outage.
