# Image shape, for any language

The runtime stage should contain the built artefact, its runtime dependencies, and nothing
else. How you get there differs by language; the shape does not.

```
deps    → resolve dependencies (cacheable, changes rarely)
build   → compile / bundle, using dev dependencies
runtime → fresh base + artefact + prod-only dependencies
```

## Copy the dependency manifest before the source

```dockerfile
COPY <manifest> <lockfile> ./
RUN <install>
COPY . .
RUN <build>
```

Reversed, every source edit invalidates the install layer and every build reinstalls
everything. This ordering is the difference between a 30-second and a 6-minute pipeline, and
it costs nothing.

| Language | Manifest + lock | Install | Prod-only install |
|---|---|---|---|
| Node | `package.json`, lockfile | `npm ci` / `pnpm i --frozen-lockfile` / `bun install --frozen-lockfile` | `--omit=dev` / `--prod` |
| Python | `pyproject.toml`, `uv.lock` / `requirements.txt` | `uv sync --frozen` / `pip install -r` | `--no-dev` |
| Go | `go.mod`, `go.sum` | `go mod download` | n/a — static binary |
| Rust | `Cargo.toml`, `Cargo.lock` | `cargo fetch` | n/a — static binary |
| Java | `pom.xml` / `build.gradle` | `mvn -o dependency:go-offline` | runtime JRE only |
| PHP | `composer.json`, `composer.lock` | `composer install` | `--no-dev` |

Always use the frozen/locked form. A resolver that is allowed to update the lockfile turns
"the build changed" into a silent event with no commit behind it.

Compiled languages (Go, Rust) collapse the runtime stage to a scratch/distroless base plus
one binary. Interpreted languages need the interpreter and the production dependency tree,
which is why the separate prod-only dependency stage earns its place there.

## Build-time versus runtime configuration

This distinction causes more confusion than anything else in the file.

**Baked at build time** — anything the build inlines into an artefact that ships to users
(a web bundle's API URL, a compile-time feature flag). It *must* be a build argument;
setting it at runtime has no effect, because the value was already substituted into the
bundle. The failure is silent: the app loads and the affected feature is broken with
`undefined` where the value should be.

**Read at runtime** — database URLs, API keys, anything secret. These arrive through the
host's env file when the container starts.

The dividing line is also the security line:

> Build arguments and `ENV` are visible via `docker history`. Anyone who can pull the image
> can read them.

So a public API URL as a build arg is fine — it ships to every browser anyway. A database
password as a build arg is a leak, regardless of which stage it is in, because build caches
and intermediate layers outlive the stage.

## Placeholders for build-time imports that demand config

Some builds import application modules to analyse them, and those modules throw at import
time if a connection string is absent. The build then fails for a resource it never uses.
A syntactically valid placeholder satisfies the check without opening a connection, because
most database clients connect lazily:

```dockerfile
ENV DATABASE_URL="postgres://build:build@127.0.0.1:5432/build"
```

Label it clearly as a placeholder. A real connection string here is a leak into the build
cache, which for hosted caches means off the machine entirely.

## Do not invoke package-manager scripts in the image

```dockerfile
RUN npm run build      # what does this actually run?
```

Wrapper scripts accumulate host assumptions — reading git state, sourcing a local env file,
setting variables from the branch name. Inside an image with no `.git` and no local env,
those degrade silently: a variable resolves to empty, the build succeeds, and you ship a
development build to production.

Invoke the underlying tool directly and set the environment explicitly in the Dockerfile.
When you do, read the script you are replacing — its contents tell you which variables the
project expects.

## Runtime stage hygiene

```dockerfile
FROM <base>            # fresh, not FROM build
ENV <MODE>=production
COPY --from=build --chown=app:app /app/<artefact> ./
COPY --from=deps  --chown=app:app /app/<prod-deps> ./
USER app
EXPOSE 8080
HEALTHCHECK ...
CMD ["<binary>", "<args>"]
```

- **Fresh base**, not `FROM build`. Otherwise source, compilers, and dev dependencies ship.
- **`--chown` at copy time**, since the next line drops privileges and a process that cannot
  write its own working directory fails in obscure ways.
- **`USER`** — a non-root user costs nothing and removes the first escalation step.
- **`EXPOSE` is documentation only.** It publishes nothing; the compose `ports:` entry does.
- **Delete build caches in the same `RUN` that creates them.** A later `RUN rm -rf` only
  hides them; the bytes stay in the earlier layer.
- **Exec-form `CMD`** (JSON array), so the process is PID 1 and receives `SIGTERM` from
  `docker stop`. Shell form wraps it in `/bin/sh`, which does not forward signals, so every
  stop waits the full timeout and then kills — slower deploys and truncated requests.
- **Bind `0.0.0.0` inside the container.** Loopback there is unreachable from the host.
