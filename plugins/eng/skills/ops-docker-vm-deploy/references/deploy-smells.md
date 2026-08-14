# Deploy smells

Review checklist. Each line is a failure that has actually happened, ordered by how quietly
it fails — the quiet ones are the expensive ones.

## Silent

- [ ] A file exists in git and on the host but CI never ships it. Edits merge, CI is green,
      the host keeps running the old copy.
- [ ] `environment:` removed from the deploy job while stale repository-level `SSH_*`
      secrets still exist → deploys to the wrong machine, run goes green.
- [ ] Build-time config supplied at runtime instead. The bundle already has `undefined`
      baked in; the app loads and one feature is quietly broken.
- [ ] Healthcheck never touches the database. Bad connection string → healthy → "success".
- [ ] Rollback prints "rolled back" without re-checking health. The log says recovered; the
      site is down.
- [ ] Deploy tag computed independently in two places. They agree until the day they do not.
- [ ] Prune without a label filter on a shared host. Someone else's stopped service loses
      its image.
- [ ] Environment name typo. GitHub creates an empty environment, every secret resolves to
      `""`, and the SSH step tries to connect to an empty host.

## Loud, but at the wrong time or with the wrong message

- [ ] `set -e` kills the script between the first mutation and the rollback block. Run is
      red, host is left mid-deploy pointing at a broken tag.
- [ ] Missing host prerequisite surfaces as a downstream error ("config file is invalid"
      when the real cause is a missing env file).
- [ ] `[ -f x ] && cmd` under `set -e` exits the script when `x` does not exist — the exact
      case the test was guarding.
- [ ] Health poll budget shorter than the healthcheck's worst-case verdict → rollback on
      timeout, not on failure.
- [ ] A referenced-but-unset SSH passphrase secret inherited from repository scope → first
      step fails with a key error that reads like a key problem.
- [ ] Checkout added to the deploy job without `contents: read`.

## Dangerous

- [ ] Published port bound to `0.0.0.0` instead of `127.0.0.1`. Plain HTTP on the public
      internet, bypassing the proxy; the firewall does not catch it.
- [ ] Directory sync to the host (`--delete`, or an action flag that clears the target).
      Deletes the host-only secrets file, which has no other copy anywhere.
- [ ] Secrets as build args or `ENV`. Readable by anyone who can pull the image.
- [ ] Secrets interpolated into the SSH `script:` body instead of passed via `envs:`. They
      can surface in logs.
- [ ] Long-lived registry token stored on a host you do not own.
- [ ] Deploy job's `if:` checks only the enable flag → opening a pull request SSHes into
      production with unmerged code.
- [ ] No memory limit on a small shared box. The OOM killer picks the biggest process, which
      is usually the neighbour's database.
- [ ] `ufw enable` before allowing SSH, on a machine with no second account.

## Wasteful

- [ ] Pull requests push images to the registry.
- [ ] Pull requests write to the layer cache and evict the branch's layers.
- [ ] `COPY . .` before the dependency install → every source edit reinstalls everything.
- [ ] Build caches left in the image (deleted in a later `RUN`, so the bytes remain).
- [ ] No log rotation. Root partition fills; the whole machine goes down, not just the app.
- [ ] Default six-hour job timeout on a step that can hang on a prompt.
- [ ] Shell-form `CMD`, so `docker stop` always waits the full timeout before killing.

## Process

- [ ] Deploy script has never been executed against any failure path.
- [ ] Rollback has never been rehearsed on the real host.
- [ ] Nobody wrote down how far back rollback actually reaches.
- [ ] Tool versions on the host were assumed rather than checked.
- [ ] The runbook says to edit a file on the host that CI now overwrites.
