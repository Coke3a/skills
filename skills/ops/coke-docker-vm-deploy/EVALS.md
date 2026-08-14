# EVALS

## Purpose

Verify the skill triggers for shipping a container to a self-administered server, does not
trigger for self-deploying platforms or orchestrators, and produces guidance that survives
the failure paths rather than only the happy path.

## Positive Trigger Prompts

- "Set up GitHub Actions to build a Docker image and deploy it to my DigitalOcean droplet."
- "I've got a Go API in a container and an empty Hetzner box. How do I wire up CI/CD?"
- "review my deploy workflow — I want to make sure a bad release rolls back"
- "my deploy said it rolled back but the site was still down, what did I get wrong"
- "ทำ CI/CD deploy docker ขึ้น VPS ยังไง"
- "we deploy by ssh-ing in and running docker compose pull by hand, help me automate it"
- "the workflow is green but the server is still running last week's build"
- "how do I get secrets onto the server without committing them"
- "our compose file on the server doesn't match the repo and nobody knows why"

## Negative Trigger Prompts

- "Set up a preview deploy for my Next.js app on Vercel."
- "Write a Helm chart for this service and set up a canary rollout."
- "My ECS task keeps failing its target group health check."
- "Add clippy and cargo test to my Rust CI." → `coke-rust-ci-cd`
- "My Dockerfile build is slow, how do I speed up the layer cache?" (image build only, no
  deploy target — borderline; acceptable either way)
- "Write the login endpoint for this API."
- "Configure nginx to serve a static site from /var/www." (no container, no pipeline)

## Expected Behavior

- Establishes the ownership boundary (which files CI ships, which the host owns) before
  producing YAML.
- Reads the target host before recommending, rather than assuming an empty machine.
- Publishes ports on `127.0.0.1` and says why the firewall is not the control.
- Keeps secrets out of build args, image layers, and the CI-owned tag file.
- Writes a rollback that restores every mutation and then verifies the result.
- Captures failures that would otherwise let `set -e` skip the rollback.
- Offers the bundled harness rather than inventing a new one.
- Distinguishes what was tested from what was not.

## Must Not Do

- Must not claim a deploy works without saying what was actually verified.
- Must not recommend `git pull` on the production host or a directory sync that could
  delete the host-only secrets file.
- Must not put secrets in build args, `ENV`, the compose file, or the CI-owned `.env`.
- Must not remove the `environment:` line or reference an unused secret name.
- Must not run commands against a client's production machine without confirmation.

## Pass Criteria

- [ ] Correct trigger decision on positive and negative prompts.
- [ ] Ownership table produced before workflow YAML.
- [ ] Rollback path restores config as well as tag, and verifies health afterwards.
- [ ] Port binding, prune scoping, and log rotation all addressed for a shared host.
- [ ] Failure paths exercised with `scripts/test-deploy-script.sh`, results reported
      honestly including what the stub does not prove.

## Harness self-check

The bundled harness is verified against two inputs and must keep discriminating between
them — if a change makes the naive script pass, the assertions have gone slack:

```bash
scripts/test-deploy-script.sh templates/deploy.sh    # expect 10/10
```

A deliberately naive script (uncaptured `up -d`, no config backup, unverified rollback,
no preflight) scores 2/10, failing exactly the cases that describe its defects.
