# <APP> — host runbook

Target: `<HOST>` · App dir: `/opt/<APP>` · User: `<USER>` · Domain: `<DOMAIN>`

Written for whoever operates this box next, who has none of the original context.

## Who owns which file

| Path | Owner | Notes |
|---|---|---|
| `/opt/<APP>/docker-compose.yml` | **CI, every deploy** | Edit in git. Changes made here are overwritten. |
| `/opt/<APP>/.env` | **CI, every deploy** | Holds `TAG=` and nothing else. Never put secrets here. |
| `/opt/<APP>/app.env` | **Host only** | Secrets. `chmod 600`. Never committed, no copy in git. |
| `/etc/nginx/sites-available/<APP>` | Host, by hand | `nginx -t` then `reload`, never `restart`. |

## First-time setup

```bash
mkdir -p /opt/<APP>
nano /opt/<APP>/app.env          # template: deploy/app.env.example in the repo
chmod 600 /opt/<APP>/app.env
stat -c '%a' /opt/<APP>/app.env  # 600
```

Do not create `.env` — the first deploy writes it.

## Deploying

Merge to `main`. The workflow builds an image, ships the compose file, and restarts
the service. Nothing is run by hand.

## Rolling back

Automatic on a failed health check. By hand:

```bash
cd /opt/<APP>
cat .env                                   # current tag
docker image ls <REGISTRY>/<ORG>/<APP>
sed -i 's/^TAG=.*/TAG=sha-XXXXXXX/' .env
docker compose up -d
docker compose ps                          # confirm healthy — do not assume
```

This restores the tag only. If that deploy also changed `docker-compose.yml`, roll the
file back in git too; the workflow's own rollback restores it, a manual one does not.

**The window is 7 days.** Successful deploys prune older images. Past that, pull it back
first — this is the only step that needs a long-lived registry token (kept in the password
manager, deliberately not on this host):

```bash
docker login <REGISTRY> -u <user>          # token with read access to packages
docker pull <REGISTRY>/<ORG>/<APP>:sha-XXXXXXX
```

## Changing a secret

```bash
nano /opt/<APP>/app.env
cd /opt/<APP>
docker compose up -d --force-recreate      # a plain restart does not reload env
```

No rebuild needed — except for values baked in at build time, which live in the workflow.

## Logs

```bash
docker compose logs --tail 200 -f
tail -f /var/log/nginx/<APP>.access.log
```

## Health

```bash
docker compose ps
docker inspect <APP> --format '{{.Config.Image}} {{.State.Status}} {{.State.Health.Status}}'
docker inspect <APP> --format '{{.State.OOMKilled}} restarts={{.RestartCount}}'
```

The healthcheck probes `<PROBE-PATH>`. Note what that does **not** cover: <e.g. it does not
touch the database, so a bad connection string still reports healthy>.

## Access

CI holds an SSH key for `<USER>` on this host. Revoke by deleting its line from
`~<USER>/.ssh/authorized_keys` — the comment on the line identifies it. Deploys will then
fail at the SSH step until a new key is installed.

## Gotchas

- `docker-compose.yml` on this host is overwritten by every deploy. Edit it in git.
- It must keep `127.0.0.1:<PORT>:<PORT>`. Without the prefix, Docker's own iptables rules
  expose the port to the internet, bypassing the firewall.
- `.env` holds only `TAG=` and is overwritten every deploy. Never put secrets there.
- `app.env` has no copy anywhere else. Back it up to the password manager.
- The deploy's file copy is scoped to one filename. Never widen it to sync the directory —
  that would delete `app.env`.
- `nginx -t` must pass before `systemctl reload nginx`; a failed reload takes the other
  sites on this machine down with it.
