# Healthchecks

The healthcheck is what turns "the deploy ran" into "the deploy worked". Its value is
entirely determined by what the probe actually touches.

## Shape

```dockerfile
HEALTHCHECK --interval=15s --timeout=5s --start-period=40s --retries=5 \
  CMD <probe> || exit 1
```

| Flag | Meaning |
|---|---|
| `--start-period` | Grace window; failures inside it do not count as retries |
| `--interval` | Time between probes |
| `--timeout` | A probe slower than this counts as a failure |
| `--retries` | Consecutive failures before the status flips to `unhealthy` |

Worst case to reach `unhealthy` ≈ `start-period + interval × retries`. For the values
above, ~115s.

## The poll budget must exceed that number

The deploy script waits for a verdict. If its budget is shorter than the worst case, it
gives up before the container has said anything, and you roll back on a timeout rather than
on a failure — indistinguishable in the log from a genuinely broken release, and it will
also fire for a release that was merely slow to warm up.

```bash
for _ in $(seq 1 40); do ... sleep 5; done      # 200s > 115s ✓
```

Keep the two numbers written near each other, in the Dockerfile and in the script, and when
you change one, change the other. This is the kind of coupling that silently rots.

## Prefer probes with no extra dependencies

Slim base images often lack `curl` and `wget`. Use something the runtime already provides:

| Runtime | Probe |
|---|---|
| Node 18+ | `node -e "fetch('http://127.0.0.1:PORT/health').then(r=>process.exit(r.ok?0:1)).catch(()=>process.exit(1))"` |
| Python | `python -c "import urllib.request,sys; sys.exit(0 if urllib.request.urlopen('http://127.0.0.1:PORT/health').status==200 else 1)"` |
| Go / Rust | Ship a `--healthcheck` subcommand in the same binary |
| Anything | Add `curl` to the runtime stage — cheap, and explicit |

## Know what your probe proves

A probe against a static page proves the process is listening. It does **not** prove the
database is reachable, the cache is up, or the migration ran. With such a probe:

- A release with a broken database connection string reports **healthy**.
- The deploy reports **success**.
- The site is broken.

That is not necessarily the wrong trade. A probe that touches the database will mark the
container unhealthy during a 30-second database blip and trigger a restart loop for a
failure the app did not cause. Shallow probes are stable; deep probes are informative.

What matters is that the choice is deliberate and written down, because it determines what
the green check means:

- **Shallow probe** → after the first deploy of a release, verify the real dependency paths
  by hand. Do not treat green as proof.
- **Deep probe** → be explicit about which dependencies can take the service down, and make
  sure the restart policy will not thrash.

A reasonable middle ground: shallow probe for the container healthcheck, and a separate
post-deploy smoke check in the workflow that exercises one real end-to-end path.

## Do not add a second healthcheck in compose

The container inherits the image's `HEALTHCHECK`. Redefining it in the compose file gives
two sources of truth whose numbers drift apart from the poll budget above. If the compose
file needs a note, leave a comment saying the check is inherited and where it lives.
