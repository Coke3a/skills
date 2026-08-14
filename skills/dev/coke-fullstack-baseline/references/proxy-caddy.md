# The proxy has four jobs, and the third one is a trap

Caddy is one container per browser-facing unit. `templates/Caddyfile` is the whole file; this
is why it is shaped the way it is.

| # | Job | The rule |
|---|---|---|
| 1 | **TLS** | the certificate is automatic — the two ACME constraints in `stack-pins.md` are the whole burden. ⚠ **`Strict-Transport-Security` is not**, and Caddy never emits it on its own |
| 2 | **Serve the built SPA** | `try_files … /index.html`, or every deep link is a `404` on refresh |
| 3 | ⚠ **Cache headers** | **hashed assets `immutable`, `index.html` never cached** |
| 4 | **Proxy `/api/v1`** | over the private network — the backend publishes **no public port** |

⚠ **Job 3 is a two-way trap and both directions are real outages.** Cache `index.html` and
users keep loading an old shell asking for asset hashes that no longer exist — a white screen
**a redeploy does not fix**, because the stale copy is in *their* browser. Fail to cache the
hashed assets and every navigation re-downloads the bundle.

⚠ **Three things about that white screen the header rule does not cover.**

| | |
|---|---|
| **No `Cache-Control` is not "not cached"** | with no header a browser is free to *heuristically* cache — commonly a fraction of the resource's `Last-Modified` age, so an `index.html` that has sat there a month gets held for days. Silence is a decision, and it is the wrong one |
| ⚠ **A service worker outranks every header here** | if one is registered it can serve the old shell from its own cache and **survive any number of redeploys**, at which point every fix on this page is inert. Check for one before believing a cache diagnosis — it is the competing explanation for exactly the same symptom |
| ⚠ **Deleting the previous build's `/assets/*` is what strands the already-stuck user** | they cannot be reached by a header change; the only thing that rescues them is the old hashed file still being there when their stale shell asks for it. **Keep the last few builds' assets on the server** — that is a deploy-step decision, so it belongs to whichever skill owns the target |

## ⚠ Four things in that file are counter-intuitive, and each was a defect before it was written down

| | |
|---|---|
| ⚠ **`header` is ordered *before* `try_files`** | so the cache headers must live **inside `handle` blocks**. A top-level `header /index.html …` matches the *original* path, before the fallback rewrite — it never fires for a deep link, which is the one case job 3 exists for. `Strict-Transport-Security` is top-level precisely because it is uniform |
| **`format filter` is a wrapper, not a format** | `wrap json` does the encoding; `fields` edits values on the way through. The regex is `"[?].*"` rather than `"\?.*"` because the parser consumes backslashes inside quotes |
| ⚠ **Find out which hop emitted the `504` before changing anything** | Caddy's `transport http` timeouts default to **unset**, so an unconfigured Caddy does **not** cut a long request — which makes "it fails at almost exactly 60s" evidence of *something else in front of it*, not of Caddy. `nginx`'s `proxy_read_timeout` and an ALB's idle timeout are both 60s by default. Reproduce against the backend directly, then against each hop, and only then set a number. Once it *is* Caddy, the knob is `reverse_proxy`'s `transport http` block — not Caddy's server-level read/write timeouts, which govern something different |
| ⚠ **Raising the timeout is a treadmill, and `response_header_timeout` measures TTFB** | a fully buffered response has TTFB == total time, so every export that grows crosses the ceiling again. **Streaming is the durable fix**: send the header immediately and flush as you go, and a proxied stream needs `flush_interval -1` or the proxy re-buffers what the handler just flushed |
| ⚠ **`trusted_proxies` is the *wrong* control, despite the name** | it is a **Caddy** option for what Caddy trusts inbound. Making the backend trust `X-Forwarded-*` is Fiber's `TrustProxy`/`TrustProxyConfig` — configure the wrong side and `c.IP()` stays wrong with nothing failing |

⚠ **The log filter is the second-best answer.** It works, but a filter is a thing somebody
can reorder or misconfigure. **The durable answer is that no secret or identifier is in a
query string at all** — then the filter is defence in depth rather than the defence.

## What development cannot reproduce

The local stack runs Vite's dev server, not Caddy. Vite does SPA fallback natively and sets
no cache headers, so **jobs 2 and 3 cannot be exercised in development at all.** That is the
real cost of making the dev server the default, accepted deliberately. The compensating
control is the post-deploy smoke test, which asserts both against the real Caddy on every
deploy.
