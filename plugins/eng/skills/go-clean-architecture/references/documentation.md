# Documentation Workflow Is Out Of Scope

This Go Clean Architecture skill does not define a general documentation workflow.

Use concise doc comments where Go convention expects them (exported types, constructors, and
interface methods — full sentences starting with the name). Beyond that, add comments only when
they clarify non-obvious architecture boundaries, such as why a type belongs in a specific layer
or why a conversion is intentionally one-way.
