# Prepare the host

Order matters. Each step has a check before and after; do them one at a time and read the
output before continuing. If the machine belongs to a client or carries someone else's
production traffic, several of these need their agreement first — those are marked.

## 0. Back up what you are about to change

If the provider offers snapshots and the cost is acceptable, take one — it is the only
outer-layer rollback. If not, a config-level backup costs nothing and covers most of what
you will touch:

```bash
tar czf /root/pre-deploy-backup.tgz /etc/nginx /etc/letsencrypt 2>/dev/null
dpkg -l > /root/pkg-before.txt      # or rpm -qa
```

Without a snapshot, every later step must be individually reversible. Note that in the
handover.

## 1. Baseline

Record before/after values so a later "was that us?" has an answer:

```bash
wc -l ~/.ssh/authorized_keys        # note the number
ss -tlnp
free -h; df -h /
ls /var/run/reboot-required 2>/dev/null && echo "REBOOT PENDING"
```

**Ask** before a pending reboot: a machine that reboots mid-work makes every subsequent
failure ambiguous. Do not reboot someone else's machine yourself.

## 2. Swap, if the box is small and has none

`[client agreement]` — it changes their machine, though it needs no restart and no service
interruption. See `references/host-exposure.md`. Costs nothing on a disk with room.

## 3. Docker

```bash
curl -fsSL https://get.docker.com -o /tmp/get-docker.sh
sh /tmp/get-docker.sh
```

If Docker is already present, do not reinstall — check the version and inspect the existing
daemon config instead of overwriting it.

Then log rotation, which is not optional (`references/host-exposure.md`):

```bash
cat > /etc/docker/daemon.json <<'JSON'
{ "log-driver": "json-file", "log-opts": { "max-size": "10m", "max-file": "3" } }
JSON
systemctl restart docker
```

Merge into the file if it already exists. Verify:

```bash
systemctl is-enabled docker         # enabled — containers must return after reboot
systemctl is-active docker
docker run --rm hello-world
```

Note that installing Docker writes iptables chains immediately. Nothing is exposed until a
container publishes a port, which is why the bind address must be right before the first
`up`, not after.

## 4. Shared network, if other services will join later

```bash
docker network create <name>
```

Declaring it `external: true` in compose means compose expects it to exist and will not
create or delete it — so a missing network fails loudly at `up` with a clear message rather
than silently creating an isolated one.

## 5. Application directory and secrets

```bash
mkdir -p /opt/<app>
```

Then create the secrets file **by typing it on the server**, not by copying it there:

```bash
nano /opt/<app>/app.env        # use app.env.example as the template
chmod 600 /opt/<app>/app.env
stat -c '%a' /opt/<app>/app.env    # 600
```

Sending secrets through chat or email puts them in someone's history forever. Use an
encrypted channel or type them.

Do not create the tag file — the first deploy writes it.

The compose file does not need to be copied by hand if CI ships it; the deploy's staging
copy creates the directory tree on its own. Copying it once anyway is harmless and lets you
run the stack manually before wiring CI.

## 6. CI key

`[client agreement]` if the key grants root — tell them in writing that CI holds it and that
revoking it means deleting one line.

Generate on **your** machine, never on the server, so the private key never exists there:

```bash
ssh-keygen -t ed25519 -N "" -C "ci-<app>" -f ~/.ssh/ci_<app>
```

`-N ""` is required: the deploy action must not be given a passphrase parameter
(`references/github-secrets-scoping.md`).

Do not reuse a key that already exists on the server. Such keys are usually for *outbound*
use (pulling from a repository), you cannot tell what else they open, reusing one means
revocation breaks something unknown, and you would have to copy a private key off the
machine to use it.

Append the public key, keeping a second session open, and check the line count moved by
exactly one:

```bash
wc -l /root/.ssh/authorized_keys        # before
cat >> /root/.ssh/authorized_keys       # paste the .pub, Ctrl-D
wc -l /root/.ssh/authorized_keys        # exactly one more
```

Test from your machine before putting the key in CI:

```bash
ssh -i ~/.ssh/ci_<app> <user>@<host> 'docker ps'
```

## 7. Reverse proxy

One new site file. Do not touch the existing ones, and do not install a second proxy
alongside the one already holding 80/443.

Two stages, because the config cannot reference a certificate file that does not exist yet:
serve `:80` first, obtain the certificate, then enable `:443`. `templates/nginx-reverse-proxy.conf`
has both, the second commented out.

```bash
nginx -t && systemctl reload nginx      # reload, never restart
```

`restart` drops every connection on the machine, including sites that are not yours.
Afterwards, confirm the other sites still answer.

## 8. Firewall

`[client agreement]` — highest-risk step here; see `references/host-exposure.md` for the
ordering and the reason a provider-level firewall is usually the safer choice.

## 9. Verify exposure after the first deploy

```bash
ss -tlnp | grep <port>                  # 127.0.0.1
nc -z -w5 <public-ip> <port>            # from outside: must fail
```

Repeat this after any change to the compose file. It is the check that catches the most
dangerous single-character mistake in the whole setup.
