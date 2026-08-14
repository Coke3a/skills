# Tags, registries, rollback window

## Deploy an immutable tag

`latest` moves. A tag derived from the commit does not:

```
ghcr.io/<org>/<app>:sha-a1b2c3d
```

Because the tag is immutable, the previous tag is a **known-good state**, and rollback is a
two-line operation with no rebuild, no git, and no thinking:

```bash
printf 'TAG=%s\n' "$PREVIOUS" > .env
docker compose up -d
```

This is the entire reason the pipeline is shaped the way it is. Anything that makes the
running artefact depend on a rebuild — `latest`, building on the host, `git pull` — destroys
it.

Digests (`@sha256:...`) are stronger still and worth using if the registry may be rewritten.
For a private registry you control, a commit-derived tag is enough and reads better in logs.

## Registry names are case-sensitive in a way that will bite you

GitHub organisations may contain uppercase letters. Docker repository names may not. So
`ghcr.io/${{ github.repository }}` interpolated directly produces
`invalid reference format: repository name must be lowercase` for an org like
`Example-Org`.

Use the metadata action, which lowercases for you:

```yaml
- id: meta
  uses: docker/metadata-action@v5
  with:
    images: ghcr.io/${{ github.repository }}
    tags: type=sha
```

Then the compose file's `image:` must use the **same lowercased** string. Hand-assembling
the tag somewhere else in the pipeline is how the two drift apart, and the symptom is a pull
failure at deploy time rather than a build failure.

`type=sha` emits `sha-<7 chars>`; `steps.meta.outputs.version` is that value, which is what
you pass to the deploy job.

## Registry auth on the host

The host needs to authenticate to pull a private image. Two options:

**Store a long-lived token on the host.** Simple, and wrong for a machine you do not own:
the credential sits in `~/.docker/config.json` indefinitely, it expires eventually and the
deploy then fails in a way nobody predicted, and someone has to remember its expiry date.

**Pass a short-lived token over SSH at deploy time.** The workflow's built-in job token
already has `packages: read`; forward it as an environment variable, log in, and log out on
exit:

```bash
echo "$REGISTRY_TOKEN" | docker login <registry> -u "$REGISTRY_USER" --password-stdin
trap 'docker logout <registry> >/dev/null 2>&1 || true' EXIT
```

Nothing persists on the host, nothing expires, nothing to document. Prefer this.

Pass the value through the SSH action's environment-variable parameter rather than
interpolating it into the script body — interpolated secrets can surface in logs.

A long-lived token is still worth creating for *manual* recovery (pulling an old tag by
hand). Keep it in a password manager, not on the box.

## The rollback window is what your prune says it is

```bash
docker image prune -af --filter "until=168h" --filter "label=<label>"
```

Seven days. Past that, rolling back needs a manual pull with the credential above. Write the
number in the runbook; "we can always roll back" is false in a way that only becomes visible
during an incident.

Note the ordering: prune runs only after a *successful* deploy, and the currently-running
image is never unused, so the immediately-previous image is still present during a failed
deploy. The window matters for deliberate rollbacks days later, not for automatic ones.

## PR builds

Build on pull requests — it is often the only automated check a small repo has — but do not
push:

```yaml
push: ${{ github.event_name != 'pull_request' }}
```

Pushing from PRs fills the registry with images nobody will ever run, and the push is the
slowest part of the job. Same reasoning for the layer cache: read it on PRs, write it only
from the default branch, so PR layers do not evict the branch layers that real deploys use.
