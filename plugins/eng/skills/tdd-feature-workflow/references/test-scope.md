# Choosing test scope

Choose the cheapest level that can fail for the behavior the user cares about.

| Needed confidence | Suitable scope |
| --- | --- |
| Pure rule, transformation, invariant, or state transition | Unit test through the stable interface |
| Orchestration across owned components | Component/service test with controlled external boundaries |
| Serialization, database mapping, transaction, filesystem, queue, or provider contract | Integration or contract test against the real boundary |
| Routing, wiring, authentication, or user-visible critical journey | Focused API/UI/end-to-end test |

Move upward only when a lower-level test cannot observe the important failure. Move downward when infrastructure noise hides the behavior being designed.

A test double is appropriate when it makes an external boundary deterministic or lets the test force a meaningful failure. It is a smell when it mirrors internal calls, requires extensive setup for private details, or makes refactoring break tests without changing behavior.
