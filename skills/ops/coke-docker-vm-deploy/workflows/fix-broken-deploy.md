# Diagnose a broken deploy

Start by classifying the failure, because the three classes have almost nothing in common.

| Symptom | Class | Go to |
|---|---|---|
| Workflow red | Loud | §1 |
| Workflow green, service broken | Silent | §2 |
| Workflow red, host in a strange state | Half-done | §3 |

## §1 Workflow red

Read the log from the **bottom**, then find the first step that failed rather than the last
line printed.

**The deploy job did not run at all.** Check the `if:` — the enable flag may be `false` or
missing, or the run may be a pull request. A skipped job is a grey check, easy to read as
passing.

**SSH step fails immediately.**

- *Connection to an empty host* → a secret resolved to `""`. Either the `environment:` line
  is missing, or the environment name does not match, or the secret was created at the wrong
  scope. See `references/github-secrets-scoping.md`.
- *Key format / "not password protected"* → a passphrase parameter is referenced somewhere,
  picking up an inherited secret; or the pasted key is truncated, or is the `.pub`.
- *Permission denied* → the public key is not in the target user's `authorized_keys`, or it
  is in a different user's.

**Image pull fails.** Compare the tag in the log with the compose file's `image:` line,
character by character. The usual causes are a case mismatch in the org name and a tag
assembled independently in two places. See `references/image-and-tagging.md`.

**Compose config invalid.** Read the actual message rather than trusting the script's
summary — a missing env file reports as a config error in many versions. Then check the
host's compose version against the one the file was written for.

**Health check never passes.** `docker compose logs --tail 200 <service>` on the host. If
the container is not even created, the problem is compose or the image, not the app.

## §2 Workflow green, service broken

The green check proved less than it appeared to. Work through what it actually verified:

- **Healthcheck too shallow.** A probe that does not touch the database reports healthy with
  a broken connection string. `docker compose exec <service> printenv | grep <VAR>` and
  compare against `app.env`. See `references/healthchecks.md`.
- **Build-time config missing.** If the broken thing is in a browser bundle, the value was
  probably supplied at runtime instead of as a build argument, and the bundle has `undefined`
  baked in. Check the browser console, not the container logs.
- **Host file never updated.** Compare the file on the host with the one in git. If they
  differ, CI is not shipping it — the silent-divergence case in
  `references/ownership-boundary.md`.
- **Deployed to the wrong machine.** Confirm the tag actually running
  (`docker inspect <name> --format '{{.Config.Image}}'`) is the one the run built. If the
  host is not even the one you expect, see the environment fallback hazard.

## §3 Workflow red, host half-done

This is `set -e` skipping the rollback (`references/rollback-safety.md`). Stabilise first,
diagnose second.

```bash
cd /opt/<app>
cat .env                                   # which tag does the host think it runs?
docker compose ps
docker inspect <name> --format '{{.Config.Image}} {{.State.Status}} {{.State.Health.Status}}'
ls -la docker-compose.yml*                 # a leftover .prev means it stopped mid-flight
```

Recover by hand:

```bash
[ -f docker-compose.yml.prev ] && mv -f docker-compose.yml.prev docker-compose.yml
printf 'TAG=%s\n' "<last-known-good>" > .env
docker compose up -d
docker compose ps                          # confirm healthy — do not assume
```

If the old image was pruned, pull it back — this is the one situation needing the long-lived
registry credential (`references/image-and-tagging.md`).

Then fix the script so the same failure reaches the rollback next time, and add the scenario
to the harness (`workflows/test-deploy-script.md`) so it stays fixed.

## Before closing it out

Two questions worth answering explicitly:

- **Would this have been caught by a test?** If yes, add the scenario to the harness. The
  failure paths are cheap to test and are exactly the ones nobody exercises by hand.
- **Did the log say what actually happened?** A message naming the wrong cause costs the
  next person more than the bug did. Fix the message too.
