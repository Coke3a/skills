# Add a route group / endpoint surface

Use this when adding a new route group (new API surface or a new feature group under an existing
surface) while preserving handler boundaries.

## 1. Choose the surface

- [ ] Pick a traffic boundary or API surface first: `publicapi`, `adminapi`, `webhook`,
      `dashboard`, or the project equivalent
- [ ] Create `internal/handler/router/{surface}/` if it does not exist (one package per surface)

## 2. Add the handler

- [ ] Add `{Entity}Handler` struct with injected usecases in
      `internal/handler/router/{surface}/{entity}.go`
- [ ] Define request/response DTOs in the same file/package
- [ ] Add `Register(r fiber.Router)` declaring only this handler's routes
- [ ] Handler methods stay thin: bind, map, call usecase, map, return

## 3. Wire the group

- [ ] In `internal/handler/app/routes.go`, create or reuse the group
      (`v1 := app.Group("/api/v1")`, `admin := v1.Group("/admin", authMiddleware)`)
- [ ] Attach surface-level middleware (auth, rate limit) at the group, not per route
- [ ] Call the handler's `Register` on the group
- [ ] Extend the `RouterDeps` struct and `cmd/api/main.go` wiring

## 4. Verify

- [ ] Route registration lives in handler `Register` methods; `routes.go` only composes groups
- [ ] No business logic, SQL, or provider SDK calls entered the handler layer
- [ ] Middleware order still starts with `recover` and `requestid` before `logger`
- [ ] `go build ./...`, `go vet ./...`, `go test -race ./...` pass
