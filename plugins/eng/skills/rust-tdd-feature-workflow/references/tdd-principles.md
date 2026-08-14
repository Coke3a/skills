# TDD Principles

- Use red/green/refactor: failing test, smallest passing implementation, cleanup while green.
- Keep a short test list before coding; pick one item at a time.
- Work in small batches so each behavior drives one design decision.
- Write one behavior-focused test at a time.
- Design through public or intended interfaces.
- Treat tests as executable documentation of expected behavior.
- Refactor as a real step, not an optional cleanup after large changes.
- Confirm failing tests fail for the expected reason before implementing.
- Do not create every test level upfront.

TDD does not replace integration, security, performance, exploratory, or usability testing. Add
those only when they provide distinct confidence for the behavior being changed.
