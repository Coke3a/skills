# Review Priorities

Use severity to communicate merge risk.

## Blocker

Must fix before merge:

- Security issue
- Data loss
- Broken behavior
- Architecture violation that will spread
- Failing tests/build
- Panic from user input
- Wrong authorization
- Likely deadlock
- Blocking operation inside async handler that can starve runtime
- Unbounded memory/task growth

## High

Should fix before merge:

- Likely bug
- Missing critical test
- Wrong error mapping
- Repository/API contract mismatch
- Usecase contains infra/HTTP dependency
- Handler contains business logic
- Serious async/concurrency footgun
- DB pool exhaustion risk
- Ignored background task failure where failure matters

## Medium

Fix if in scope:

- Maintainability issue
- Unnecessary abstraction
- Unclear naming
- Duplicate logic
- Brittle tests
- Incomplete edge cases
- Performance footgun on likely hot path
- Broad lock scope or unbounded queue in lower-risk path

## Low

Minor improvement:

- Minor clarity issue
- Small naming improvement
- Small documentation improvement
- Minor avoidable clone outside hot path

## Nit

Optional polish.
