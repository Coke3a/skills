# syntax=docker/dockerfile:1.7
# Language-agnostic shape. Replace every <...> and pick the install commands for your
# stack from references/dockerfile-patterns.md.
#
# Compiled languages (Go, Rust): drop the prod-deps stage and copy a single binary
# into a scratch/distroless runtime.

ARG RUNTIME_VERSION=<pinned-tag>

# ---- deps: all dependencies, including those needed only to build -------------------
FROM <base>:${RUNTIME_VERSION} AS deps
WORKDIR /app
COPY <manifest> <lockfile> ./
RUN <install-all>

# ---- prod-deps: runtime dependencies only, copied into the runner ------------------
FROM <base>:${RUNTIME_VERSION} AS prod-deps
WORKDIR /app
COPY <manifest> <lockfile> ./
RUN <install-production-only>

# ---- builder --------------------------------------------------------------------
FROM deps AS builder
WORKDIR /app

# Values inlined into a shipped bundle at build time, not read at runtime.
# Visible in `docker history` — public configuration only, never secrets.
ARG PUBLIC_API_URL=""

# Placeholder for a build-time import that refuses to load without one.
# Never a real connection string: build caches outlive the stage.
ENV DATABASE_URL="postgres://build:build@127.0.0.1:5432/build" \
    <MODE>=production

COPY . .

# Call the build tool directly rather than a package.json/Makefile script — wrapper
# scripts often read host state (git branch, local env) that does not exist here.
RUN <build-command> \
    && rm -rf <build-cache>

# ---- runner: fresh base, no source, no dev dependencies --------------------------
FROM <base>:${RUNTIME_VERSION} AS runner
WORKDIR /app

ENV <MODE>=production

COPY --from=prod-deps --chown=<user>:<user> /app/<deps-dir> ./<deps-dir>
COPY --from=builder   --chown=<user>:<user> /app/<artefact> ./<artefact>

USER <user>
EXPOSE <PORT>

# Worst case to `unhealthy` ≈ 40 + 15×5 ≈ 115s. The deploy script polls for 200s;
# keep that budget larger than this number.
HEALTHCHECK --interval=15s --timeout=5s --start-period=40s --retries=5 \
  CMD <probe-using-a-tool-the-runtime-already-has> || exit 1

# Exec form: the process is PID 1 and receives SIGTERM from `docker stop`.
# Bind 0.0.0.0 inside the container — loopback here is unreachable from the host.
CMD ["<binary>", "<args>", "--host", "0.0.0.0", "--port", "<PORT>"]
