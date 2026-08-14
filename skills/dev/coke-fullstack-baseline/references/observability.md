# Observability

Three stores, three jobs. **The application writes to `stdout` and exposes `/metrics`;
everything else is collection.**

```text
backend ─┬─ /metrics ──────── Prometheus ─┐
         ├─ stdout ─── Alloy ─── Loki ────┼── Grafana
         └─ OTLP ─────────────── Tempo ───┘
```

## Retention

| Store | Holds | Retention | Why |
|---|---|---|---|
| Prometheus | numbers, no people | **90 days** | least risk, most value over time |
| Loki | log lines | **14 days** | **the riskiest store** — short enough that a mistake expires on its own |
| Tempo | spans | **7 days** | largest by volume, and nobody reads a three-week-old span |

⚠ **Not one of those three numbers is a default, and each needs configuration this skill does
not carry.** Prometheus keeps 15 days and takes its retention as a **command-line flag**, not
from `prometheus.yml`. Loki keeps data **forever** until a compactor is enabled and pointed
somewhere — the shipped single-binary config has no retention settings at all. Tempo's
default is longer than 7 days, and the setting **moved** in its 3.x line, so every 2.x example
is the wrong shape. ⚠ **And none of the three has a volume anywhere in this skill** — the
pins file spends a paragraph on Caddy's stateful `/data` while the store meant to hold 90 days
gets nothing. Write these down in `observability/` before believing the table above.

## ⚠ Labels are the leak surface

A label is a dimension, kept for the whole retention window, in a store with **no access
control and no tenant scoping**.

- **Never a person.** No user id, account id, e-mail, or any customer identifier. A metric
  store is no more protected than a log file.
- **`http.route` is the route template, never the resolved path.**
  `/api/v1/<resource>/{id}` — not `/api/v1/<resource>/9f3c…`, which is both an identifier in
  a label *and* one time series per record.
- **Loki labels are `service`, `level`, `env`, `instance`. That is the list.** `request_id`
  goes **in the line**, never in a label — it is unbounded cardinality and Loki indexes labels.
- **No tenant/customer label by default.** It is one series per customer, it hands anyone with
  a Grafana login an activity profile, and it grows without bound. Adding one later is a
  deliberate decision.

⚠ **`/metrics` must not be reachable from the internet.** It is an unauthenticated endpoint on
the service that authenticates everything else, and it publishes per-endpoint request and
error counts. Bind it to the private interface, or put a scrape credential on it.

## ⚠ What tracing puts in the third store without being asked

⚠ **Check these against the instrumentation you actually install, not against this table.**
OpenTelemetry renamed its HTTP attributes — anything written against `http.url` /
`http.target` / `db.statement` is describing a previous convention, and a guide using those
names is out of date in a way that looks fine.

| What leaks | Where it shows up |
|---|---|
| ⚠ **the query string, in two places at once** | its own attribute **and** again inside the full-URL attribute. The logging rule lands here too, and "strip it" is therefore **two** fixes, not one |
| ⚠ **the caller's identity** | server instrumentation commonly sets an end-user attribute from the `Authorization` header when it can read one. **That is a person in the trace store** — and the label rules above govern metric and log *labels* only, so nothing else catches it |
| ⚠ **the client IP** | usually on by default. Personal data in most jurisdictions, kept for the whole window |
| **query text and its parameters** | parameter capture is off by default in the pgx instrumentation this stack uses, so the usual warning does not apply — but **non-parameterized SQL inlines its literals into the text.** Everything here is parameterized (sqlc), which is why that holds; it is a property to keep, not a coincidence |
| **span names** | the route template is what the label rules require. Some instrumentation gets this right already — **verify rather than reimplement it** |

**Sample at 100% until volume says otherwise.** A trace store sampled at 1% cannot answer
*"what happened to this one request"*, which is the only question it exists to answer.

## ⚠ The join key, and the thing that is not an audit log

**Put `request_id` and `trace_id` on every log line, and return the request id to the
browser.** Without them there are three stores and no way to join any of it to the string a
user is reading off their error screen.

⚠ **They are two fields, not one — do not try to make the request id *be* the trace id.** A
trace id is minted by the tracing SDK in its own format, and an incoming request that already
carries one **inherits it**, so the service does not get to choose the value. Meanwhile
tracing-off is a valid state and `X-Request-Id` is required on **every** response — which
means the request id has to exist when there is no trace at all. Mint it unconditionally, log
both, and leave `trace_id` empty when tracing is off.

> ⚠ **An observability store is not an audit log.** It is best-effort, sampled, lossy,
> retained on a rolling window, and readable by everyone with a Grafana login. An audit record
> is transactional, complete, access-scoped, retention-governed, and readable by one audience.

**No compliance obligation is ever discharged by a log line.** If the question is *"who read
this record"*, the answer is a table written by the application in the same transaction as the
read — and it must remain the answer if Loki is down, purged, or was never deployed. **And the
converse: an audit row is not an alert.** Grafana does not query the audit table; the
application emits a *metric* beside the audit write, and the metric is what alerts.

The failure this prevents: *"we have centralised logging now, so we can log the access
there"* — which moves a compliance record into a system that samples, expires, and is widely
readable, without anything failing.

## Health checks — "running" is not "working"

Three checks, not one:

| Check | Asserts |
|---|---|
| **Liveness** | the process is up |
| **Readiness** | **each database role connects as itself.** A readiness check on one role says nothing about the others, and a broken auth-path role is a total outage the main role cannot see |
| **Correctness assertion** | one query whose *wrong* answer is silent — see below |

⚠ **Name the three paths, and put them where the proxy does not swallow them.** Every path
the proxy does not route to the backend falls into the SPA fallback and returns **`200` with
`index.html`** — so a health check at a path nobody registered *passes*, against a dead
backend, and the smoke test passes with it.

Point the external uptime check at a page that renders without a session. Every other route
redirects.

## Correctness alerts vs performance alerts

Two kinds of alert, and confusing them is how the good ones go quiet.

| | error rate · latency · CPU · disk | the silent-failure alerts |
|---|---|---|
| Who notices first | **the user** — it is slow or broken, and they call | **nobody** — the screen is blank, or the record silently is not there |
| What the number means | **quantity** — 3% tolerable, 5% not | **correct or not** — there is no acceptable level |
| Threshold | tuned | **`> 0`, ever** |

Baseline alert set: error rate > 5% · p95 latency > 1s · container restarts above threshold ·
disk > 80% · **certificate expiry at 14 days** · database connections > 80% of the cluster
limit.

⚠ **Only the first two of those come from the application.** The diagram above has exactly one
arrow into Prometheus — the backend's own `/metrics`. Restarts, disk, database connections and
certificate expiry each need **a separate exporter or a scrape target that nothing here names
and nothing pins**, and the proxy's metrics endpoint is off by default besides. Four of six
alerts are therefore configured against data that does not exist yet. Deciding the exporters
is `observability/`'s job; knowing that they are missing is this file's.

⚠ **And the right-hand column of that table has no entry anywhere.** Every alert listed above
is a *threshold* alert — the left column. The `> 0` column is called *"the two best signals in
the system"* and then never instantiated, which is a definition pointing at a definition.
**The shape is a counter that only increments when an invariant is violated, alerted above
zero, with the health check running the same assertion** — the invariant itself is the
project's, and naming even one is what turns this section from a category into a gate.

⚠ **Keep the certificate alert even though Caddy renews automatically — automation is the
reason to keep it, not to drop it.** With manual renewal a human has the date in a calendar.
With Caddy nobody is watching, so the alert stops meaning *"somebody forgot"* and starts
meaning *"renewal has been failing silently"* — ACME challenges fail on a DNS change, a
firewall rule, a rate limit, or a lost `/data` volume, and Caddy retries quietly for weeks
before the certificate actually expires. **Fourteen days is the window in which that is still
a ticket rather than an outage**, and an expired certificate on an HSTS host is an outage with
no click-through.

⚠ **Decide what happens when a `> 0` alert fires *before* the first one fires.** If these are
triaged like ordinary tickets during early development, the team learns to ignore them within
a week — and the two best signals in the system go quiet while still technically configured.
**An alert routed nowhere is a dashboard nobody opens.**

## The connection budget

Count every database connection against a stated ceiling — application pools, migration,
`psql`, and any exporter. **Hold two in reserve for migration and manual access.** A metrics
exporter that opens its own connections is easy to add under `observability/` and no less
costly for sitting there; it needs an amendment to the budget, not a shrug.
