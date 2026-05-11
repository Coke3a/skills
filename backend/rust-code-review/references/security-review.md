# Security Review

- Do not log secrets, tokens, passwords, API keys, session cookies, authorization headers, or private credentials.
- Do not log PII unless explicitly intended and safe.
- Check ownership and authorization where records are user- or tenant-scoped.
- Avoid IDOR-style bugs where users can access records they do not own.
- Validate user input at the correct boundary/domain layer.
- Avoid SQL injection through Diesel query builder and typed parameters.
- Return safe, stable error messages to API users.
- Do not return internal infra details to API users.
- Avoid panics from user input.
- Note missing rate limiting or abuse protection when relevant to the changed behavior.
- Preserve tenant/user isolation in async and background jobs.
- Ensure concurrent code paths do not bypass authorization.
- Avoid `unsafe`; if present, require clear safety comments and tests.
