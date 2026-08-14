# Host exposure and neighbours

Most VMs in this pattern are shared: the app lands next to a database, someone else's
service, a reverse proxy, and whatever the previous contractor left. These rules exist
because the container is a guest.

## The most dangerous line in the compose file

```yaml
ports:
  - "127.0.0.1:3000:3000"     # host_ip : host_port : container_port
```

Written as `"3000:3000"`, the host IP defaults to `0.0.0.0` and the app is on the public
internet at port 3000 — plain HTTP, bypassing the reverse proxy and TLS entirely.

**A firewall does not save you.** Docker installs its own rules in the `nat` table, which
packets traverse *before* reaching the `filter` chain where `ufw` operates. `ufw deny 3000`
appears to apply, `ufw status` looks correct, and traffic still reaches the container. This
is not a misconfiguration; it is how Docker's port publishing works.

Binding to `127.0.0.1` means the kernel never accepts the connection on an external
interface. The reverse proxy on the same host reaches it over loopback; nothing else can.

Verify after every deploy, from both sides:

```bash
ss -tlnp | grep 3000                     # on host: must show 127.0.0.1, not 0.0.0.0
nc -z -w5 <public-ip> 3000               # from outside: must fail
```

Inside the container the process must still bind `0.0.0.0` — the container has its own
network namespace, and binding loopback there makes it unreachable even from the host.
The two `127.0.0.1`s are at different layers; do not "fix" one to match the other.

## Pruning on a shared box

```bash
docker image prune -af --filter "until=168h" --filter "label=<your-label>"
```

Without the label filter, `-a` removes every unused image **on the machine**, including
images belonging to services that merely happen to be stopped. On a box you share with a
client's other apps, that is a self-inflicted outage with your name on it.

The registry metadata action already attaches a source label to your images; filter on it.
If the label does not match, the failure mode is "nothing gets deleted", which is the safe
direction. Verify with `docker image inspect <image> --format '{{.Config.Labels}}'`.

## Log rotation

The default `json-file` driver has **no size limit**. A service that logs every request
fills `/var/lib/docker/containers/...` on the root partition, and a full root partition
takes down the whole machine — the proxy cannot write logs, the database cannot write, SSH
may not let you in to fix it.

Set it in two places, because they cover different things:

```json
// /etc/docker/daemon.json — default for every container on the host
{ "log-driver": "json-file", "log-opts": { "max-size": "10m", "max-file": "3" } }
```

```yaml
# compose — explicit for this service, survives a daemon config someone else changes
logging:
  driver: json-file
  options: { max-size: "10m", max-file: "3" }
```

## Memory limits

```yaml
mem_limit: 1g
```

A cgroup limit means the kernel kills *your* process when *you* leak. Without it, the OOM
killer picks its victim by memory footprint across the whole machine — and the largest
process on a small box is usually the database, not the newcomer. Setting a limit converts
"the client's database died" into "our container restarted".

Restart policy `unless-stopped` then brings it back, while still honouring a deliberate
`docker stop` (unlike `always`, which restarts even after you intentionally stopped it to
investigate).

If containers restart on a cycle, check whether the kernel is the cause before blaming the
app:

```bash
docker inspect <name> --format='{{.State.OOMKilled}} restarts={{.RestartCount}}'
```

## Swap

A small VM with no swap has no cushion: memory pressure goes straight to the OOM killer.
A swapfile costs nothing on a disk with room and does not require a restart:

```bash
fallocate -l 2G /swapfile && chmod 600 /swapfile
mkswap /swapfile && swapon /swapfile
echo '/swapfile none swap sw 0 0' >> /etc/fstab
```

Verify `/etc/fstab` parses (`findmnt --verify --fstab`) before rebooting — a malformed
line there can leave the machine unbootable.

## Enabling a firewall on a machine you do not own alone

This is the highest-risk step in the whole exercise, and the risk is not technical
sophistication — it is that getting the order wrong locks everyone out at once, including
you, with no second account to recover through.

- Get approval first. It affects every person who uses that machine.
- Open the provider's web console in another tab **before** starting. It is the only way
  back in if SSH dies.
- Keep a second SSH session open.
- Allow SSH **first**, verify with `ufw status verbose`, and only then enable.
- Never open the app's port. Exposure is controlled by the bind address, not the firewall.

A provider-level firewall (DigitalOcean Cloud Firewall, AWS security group) is usually the
better choice here: it sits above Docker's iptables rules so it holds even if a bind address
is wrong, and it can be changed from a web panel without SSH — so a mistake is recoverable.

## Installing Docker changes the firewall immediately

The install writes `DOCKER` and `DOCKER-USER` chains as soon as it finishes. On a machine
with no firewall configured, there is now a path that ignores the one you are about to
configure. Nothing is exposed until a container publishes a port — which is exactly why the
`127.0.0.1` bind must be correct *before* the first `up`, not after.
