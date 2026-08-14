# Test Scope

Choose the lowest level that proves the behavior without hiding risk.

## Domain Unit Tests

Use for value object validation, entity invariants, state transitions, and pure business rules. Do
not use database, network, HTTP, or mocks for domain tests.

## Usecase Tests

Use for orchestration, repository trait behavior, permissions/ownership, success output, and error
mapping from `DomainError` or `RepoError` into `UsecaseError`. Use hand- written fake repositories.

## Repository Integration Tests

Use for Diesel queries, row/entity mapping, insert/find/update/delete behavior, database
constraints, transactions, not found behavior, and database error mapping. Use a real test database
when practical. Do not mock Diesel to claim database behavior is covered.

## Handler/API Tests

Use for request DTO validation, response DTO shape, route wiring, auth extraction, status codes, and
`ApiError` mapping. Avoid business logic expectations beyond the API contract.

## UI/E2E Tests

Use only for critical journeys. Do not turn backend business rules into UI-only coverage.

## Cost and Confidence

Fast domain/usecase tests give tight feedback but do not prove infrastructure. Repository/API
integration tests give broader confidence but cost more setup and runtime. Avoid placing all
behavior in API/UI tests, and avoid duplicating the same business rule across every layer.
