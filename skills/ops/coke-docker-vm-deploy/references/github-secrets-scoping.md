# GitHub secret scoping

Two documented facts, one dangerous consequence.

**Fact 1.** A job can read secrets defined on a GitHub Environment only if the job declares
that environment:

```yaml
deploy:
  environment: production     # ← this line is what puts the environment's secrets in scope
```

**Fact 2.** A secret name that resolves to nothing is an **empty string**. There is no
error, no warning, no failed step. `${{ secrets.SSH_HOST }}` for a name that is not in
scope expands to `""` and the workflow continues.

## The consequence

Consider a repo that has *both* a repository-level `SSH_HOST` (left over from an old
server) and an environment-level `SSH_HOST` (the new one):

| `environment:` line | repo-level secret exists | Result |
|---|---|---|
| present | — | Deploys to the new host. Correct. |
| **removed** | no | `SSH_HOST` is `""`. Fails loudly. Annoying but safe. |
| present | — | Correct. |
| **removed** | **yes** | **Deploys to the old host. Run goes green.** |

Row 4 is the hazard. The leftover repository secret is a decoy: it converts a loud failure
into a silent success against the wrong machine. Someone tidying up an "unnecessary-looking"
line in the workflow causes a production deploy to a server nobody is watching, and the
only evidence is a green check.

Two mitigations, and you want both:

1. A comment on the line saying what it does — not "do not delete", which invites the
   question, but the mechanism: *"SSH_* are environment secrets; this line is what puts
   them in scope."*
2. **Delete the leftover repository-level secrets** once the migration is done. That
   removes the decoy and permanently reduces row 4 to row 2.

## Never reference a secret you do not intend to use

The same emptiness rule bites in the opposite direction. If a workflow contains

```yaml
passphrase: ${{ secrets.SSH_PASSPHRASE }}
```

and that name exists anywhere in scope, the SSH action will try to decrypt the key with it.
Against a key that has no passphrase, that fails at the very first step with an error
(`key is not password protected`) that reads like a key problem, not a config problem.

So: generate the CI key with no passphrase (`ssh-keygen -N ""`) and do not reference the
parameter at all. An unreferenced secret cannot leak into the run.

## What to scope where

| Value | Where | Why |
|---|---|---|
| Host, user, private key | Environment | Machine-specific; the environment is also where approvals attach |
| Registry token | Not stored — use the workflow's built-in job token | Short-lived, scoped, rotates itself |
| Public build-time config baked into a client bundle | Repository **variable** | Not secret; it ships to every browser by definition |
| A deploy on/off switch | Repository **variable** | Read in `if:`, which is evaluated *before* the job reaches its environment |

That last row is a real constraint, not a style choice: a job-level `if:` condition cannot
read environment secrets, because the environment is not resolved until the job starts. Any
value that gates whether the job runs at all must be a repository-level variable.

## Environments are created implicitly

Referencing an environment name that does not exist does not fail — GitHub creates an empty
one on the first run. So a typo (`Production`, `prod`) yields a real environment with zero
secrets, and every `secrets.*` lookup returns `""`. The symptom is an SSH action trying to
connect to an empty host string. Create the environment in the UI first, and match the name
exactly.

## Permissions

Grant per job, not per workflow:

```yaml
build:
  permissions: { contents: read, packages: write }
deploy:
  permissions: { contents: read, packages: read }
```

`contents: read` is required by the deploy job only if it checks out the repo — which it
does as soon as you ship a config file or read the deploy script from the checkout. Adding
a checkout step without adding the permission produces a confusing early failure.

## Who can configure this

Managing secrets, variables, environments, and required status checks needs the **Admin**
role on the repository. Write and Maintain are not enough — they allow pushing and merging
but not touching Actions configuration. If someone is setting this up for the first time,
have them ask for Admin once rather than discovering the gap four separate times.
