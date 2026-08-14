# Test the deploy script without a server

The rollback path is the part of a deploy script that has never run. You can exercise every
failure path in seconds, on a laptop, with no Docker daemon and no server — by putting a
fake `docker` on `PATH` and running the real script against a temporary directory.

This finds real bugs. In the session that produced this skill it caught two: a mutation that
`set -e` skipped past on registry-pull failure, and the same on login failure — both in code
that had been reviewed and looked correct.

## Use the bundled harness

```bash
scripts/test-deploy-script.sh <path-to-deploy.sh>
```

It runs the matrix below and prints a pass/fail table. Adapt the container name, service
name, and paths at the top of the script if the project uses different ones.

## What it checks

For each scenario it asserts four things about the host directory afterwards: the tag file,
the config file, the exit code, and that the secrets file still exists.

| # | Scenario | Expected |
|---|---|---|
| 1 | Everything works | exit 0 · new tag · new config |
| 2 | Shipped config invalid | exit 1 · **old** tag · **old** config · "nothing was deployed" |
| 3 | `docker compose up` fails | exit 1 · old tag · old config · rolled back and verified |
| 4 | Container reports unhealthy | exit 1 · old tag · old config · rolled back and verified |
| 5 | Rollback container also unhealthy | exit 1 · **"ROLLBACK FAILED"** in the output |
| 6 | Rollback `up` itself fails | exit 1 · "ROLLBACK FAILED" |
| 7 | No previous tag (first deploy) | exit 1 · a message saying manual attention is needed |
| 8 | Registry pull fails | exit 1 · old tag · old config |
| 9 | Registry login fails | exit 1 · old tag · old config |
| 10 | Host prerequisite missing | exit 1 · the error names the *actual* missing file · nothing mutated |

Scenarios 3, 5, 6, 8 and 9 are the ones that catch `set -e` skipping the rollback. 2 and 10
catch inaccurate error messages. 5 and 6 catch a rollback that reports success without
verifying.

## How it works

`scripts/docker-stub.sh` dispatches on the first two arguments (`compose up`, `inspect`,
`login`, …) and returns whatever the scenario's environment variables ask for. It counts
`compose up` invocations so the first call (deploy) and the second (rollback) can be given
different outcomes — that is what makes scenarios 5 and 6 possible.

The harness copies the real script, rewrites only the application directory to point at a
temp dir, and runs it. Nothing else is changed, so what runs is the deployed script.

## Validate the config file against the real tool as well

`docker compose config` runs entirely client-side and needs no daemon, so if the Docker CLI
is installed you can validate the actual compose file for real. The stub supports delegating
just that one subcommand:

```bash
REAL_CONFIG=1 scripts/test-deploy-script.sh <path-to-deploy.sh>
```

Worth doing against the **host's** compose version when it differs from yours — Compose v2
and v5 do not accept identical files, and finding that out during a deploy is expensive.

## Also run the linters

```bash
bash -n deploy.sh          # syntax
sh -n deploy.sh            # POSIX-clean, in case the remote shell is dash
shellcheck deploy.sh       # catches the set -e interactions this is all about
```

Adding shellcheck as a pull-request job is worth it once the script is a real file, because
these are exactly the bugs it is good at.

## What this does not prove

The stub returns what you told it to. It proves the script's control flow — which branch
runs, what state is left behind — not that real Docker behaves the way the stub does. Say so
when reporting results, and rehearse a real rollback on the host before the service carries
traffic.
