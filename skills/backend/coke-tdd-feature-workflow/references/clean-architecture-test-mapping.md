# Clean Architecture Test Mapping

Use this with the companion Rust clean architecture skill. The architecture skill owns layer
structure, naming, error flow, repository trait shape, and Diesel implementation patterns.

## Domain

Test value objects, entities, invariants, state transitions, and pure rules.

Generic names:

- `ExampleEntity`
- `ExampleEntityId`
- `ExampleEntityName`

## Usecases

Test orchestration, repository trait behavior, error semantics, permissions, and ownership.

Generic names:

- `ExampleRepository`
- `FakeExampleRepository`
- `CreateExampleEntityUseCase`
- `CreateExampleEntityInput`
- `CreateExampleEntityOutput`

## Infra Repositories

Test Diesel mapping, database constraints, transactions, and query behavior with integration tests.

Generic names:

- `ExamplePostgres`
- `ExampleRepository`

## Handlers/API

Test request DTO mapping, response DTO shape, status codes, `ApiError` mapping, route wiring, and
auth extraction. Do not move business logic into handlers to make API tests pass.
