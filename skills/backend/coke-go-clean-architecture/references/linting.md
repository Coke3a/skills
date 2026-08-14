# Linting Workflow Is Out Of Scope

This Go Clean Architecture skill does not define lint configuration, lint policy, pre-commit
hooks, or CI enforcement.

The only lint commands owned by this skill are the downstream final verification commands:

```sh
go vet ./...
golangci-lint run   # when the project has a golangci-lint config
```

The assumed golangci-lint baseline follows the uber-go guide: `errcheck`, `goimports`, `revive`,
`govet`, `staticcheck`.

Use a dedicated CI/CD or linting skill for lint policy.
