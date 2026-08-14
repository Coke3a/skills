# Secrets, environments, permissions

Read `references/github-secrets-scoping.md` first — this file is the procedure, that one is
the reasoning.

## Check what already exists before adding anything

Especially on a repo that deployed somewhere else before. A leftover repository-level secret
with the same name as the one you are about to create is the decoy described in the
reference: it turns "the environment line was deleted" from a loud failure into a green
deploy against the old machine.

```bash
gh secret list   -R <org>/<repo>
gh variable list -R <org>/<repo>
gh api repos/<org>/<repo>/branches/main/protection
```

Record what is there. Do **not** delete the old secrets yet — anything still pointing at the
old host may still be in use. Deleting them is a scheduled step after cutover, and it is
what permanently closes the hazard.

These commands need the **Admin** role. If they return 404 with a valid token, that is a
permissions answer, not a networking one — Write and Maintain are not enough for Actions
configuration. Ask for Admin once, up front, rather than discovering the gap at each of the
four steps below that need it.

## Create the environment

Web UI: **Settings → Environments → New environment**, or:

```bash
gh api repos/<org>/<repo>/environments/production -X PUT
```

The name must match the workflow's `environment:` value exactly, including case. A mismatch
does not error — GitHub creates an empty environment on first use and every secret lookup
returns an empty string.

Consider enabling required reviewers if the key grants root on someone else's machine. It
turns every deploy into a deliberate act, which is proportionate when the blast radius is
another company's server.

## Add the secrets to the environment

```bash
gh secret set SSH_HOST        --env production --body "<host>"
gh secret set SSH_USER        --env production --body "<user>"
gh secret set SSH_PRIVATE_KEY --env production < ~/.ssh/ci_<app>
```

Via the web UI, these must be added from **inside the environment's page**, in its
*Environment secrets* section — not from the repository secrets page.

Pasting the private key by hand is where this usually goes wrong. Copy it programmatically
(`pbcopy < ~/.ssh/ci_<app>` on macOS) rather than selecting it in a terminal, so nothing is
truncated or re-wrapped. It must include both the `-----BEGIN` and `-----END` lines, and it
must be the file **without** the `.pub` extension.

**Do not create a passphrase secret**, and do not reference one in the workflow. See the
reference for why an unset-but-referenced name is worse than no name at all.

## Add the deploy flag as a repository variable

```bash
gh variable set DEPLOY_ENABLED --body false
```

A variable, not a secret — its value is not sensitive and it must be readable from `if:`,
which is evaluated before the job reaches its environment. Start `false`: merge the
pipeline, watch the build job pass, then flip it to `true` as a separate decision.

## Verify the scoping

On the repository's Actions secrets page you should see two distinct groups:

```
Environment secrets
  production        3 secrets      ← the new ones
Repository secrets
  <old secrets>                    ← untouched
```

If a new name appears under *Repository secrets*, it went to the wrong scope. Delete and
redo it.

```bash
gh secret list --env production
```

## Job permissions

```yaml
build:
  permissions: { contents: read, packages: write }
deploy:
  permissions: { contents: read, packages: read }
```

Grant per job. The deploy job needs `contents: read` as soon as it checks out the repository
— which it does the moment it ships a config file or reads the deploy script from the
checkout.

## Branch protection and required checks

The ordering here trips people up: a status check can only be marked required after GitHub
has **seen it run at least once**. So:

1. Merge the workflow.
2. Let the build job run once on the default branch.
3. Settings → Branches → require that check.

Doing it in the other order leaves you searching a list that does not yet contain the name.

One constraint if you make a check required: do not add path filters to its `pull_request`
trigger. A pull request that touches no matching path never reports the check at all, and
the pull request then waits forever for a result that will never arrive.

## After cutover

Delete the old repository-level secrets. Until then, the hazard in
`references/github-secrets-scoping.md` is live, and the only thing standing between a
tidy-up commit and a deploy to the wrong machine is one line of YAML.
