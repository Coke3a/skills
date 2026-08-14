# Workflow shape

Two jobs. `build` produces an image and outputs its tag; `deploy` consumes the tag.

## Triggers

```yaml
on:
  push:            { branches: [main] }
  pull_request:    { branches: [main] }
  workflow_dispatch:
```

Pull requests build without pushing or deploying — that gives a pre-merge gate for free,
which matters most in repos with no test suite, where the build is the only automated check.

`workflow_dispatch` is a manual escape hatch. Note that it can be triggered from any branch,
so if you want deploys to come only from the default branch, say so explicitly:

```yaml
if: github.ref == 'refs/heads/main' && github.event_name != 'pull_request' && vars.DEPLOY_ENABLED == 'true'
```

## Concurrency: different rules per event

```yaml
concurrency:
  group: ${{ github.event_name == 'pull_request' && format('pr-{0}', github.event.number) || 'deploy-production' }}
  cancel-in-progress: ${{ github.event_name == 'pull_request' }}
```

Two independent decisions, both load-bearing:

- **Group.** One shared group would put pull requests in the deploy queue, so PRs block each
  other and block deploys. Per-PR groups keep them independent.
- **Cancellation.** A cancelled deploy is a deploy interrupted at an arbitrary line — image
  pulled, tag file written, container half-recreated — with no rollback because the script
  was killed. Never cancel the deploy queue. Cancelling superseded PR builds is free, since
  nobody wants the results of a commit that has already been replaced.

## Gate the deploy job on two conditions

```yaml
deploy:
  needs: build
  if: github.event_name != 'pull_request' && vars.DEPLOY_ENABLED == 'true'
```

The event check is not redundant with the flag. With only the flag, **opening a pull request
SSHes into the production host** using unmerged code. It will eventually fail — the PR never
pushed an image, so the pull fails — but it fails *after* connecting and running commands
on the server. Anyone can open a pull request.

The flag itself is worth having: it lets you land and exercise the whole pipeline with
deploys off, then enable it as a separate deliberate act. Keep it a repository **variable**,
not a secret, because `if:` is evaluated before the job reaches its environment.

## Timeouts

```yaml
timeout-minutes: 30    # build
timeout-minutes: 15    # deploy
```

The default is six hours. An SSH step that hangs on a prompt will happily consume all of it,
and — because the deploy concurrency group is serialised — block every subsequent deploy
behind it.

## Passing the tag between jobs

```yaml
build:
  outputs:
    tag: ${{ steps.meta.outputs.version }}

deploy:
  needs: build
  env:
    IMAGE_TAG: ${{ needs.build.outputs.tag }}
```

Recomputing the tag in the deploy job is a chance for the two to disagree. Compute once,
pass it.

## Keep the remote script in a file

Once the remote script has a rollback path it is long enough that living inside a YAML
string costs real things: no shell linting, no syntax highlighting, no way to run it under
a test harness, and diffs mixed into YAML indentation.

`appleboy/ssh-action` accepts `script_path:`, which reads a file **from the runner's
checkout** and pipes its contents to the remote shell. Nothing is stored on the host, the
script and the image always come from the same commit, and the file can be linted and
tested like ordinary code:

```yaml
- uses: actions/checkout@v4
- uses: appleboy/ssh-action@<pin>
  env:
    IMAGE_TAG: ${{ needs.build.outputs.tag }}
  with:
    host:     ${{ secrets.SSH_HOST }}
    username: ${{ secrets.SSH_USER }}
    key:      ${{ secrets.SSH_PRIVATE_KEY }}
    envs: IMAGE_TAG
    script_path: deploy/deploy.sh
```

The script file must be committed. Referencing a path that is untracked produces a deploy
failure that looks nothing like "you forgot to `git add`".

Give the file a header comment saying it runs on the remote host and which variables arrive
through `envs:` — standing alone, nothing else in it says so.

## Shipping files to the host

The SCP action runs `mkdir -p` on its target, so a staging directory needs no manual
creation — and on a fresh host it will create the application directory as a side effect.

Copy step ordering matters: it runs *before* the SSH step, so back-up-then-install logic
must live inside the script, with the copy landing in a staging directory. See
`references/ownership-boundary.md`.

## Pin actions

Pin to a released tag at minimum. Read the action's inputs before using a new one — some
have destructive options (a flag that clears the target directory before uploading) that
must never be enabled against a directory holding host-only secrets.
