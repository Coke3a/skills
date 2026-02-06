// Example: Error mapping chain in the actual codebase
//
// DomainError -> UsecaseError (From impl, auto via ?)
// RepoError   -> UsecaseError (From impl, auto via ?)
// UsecaseError -> ApiError    (From impl, auto via ?)
// ApiError    -> HTTP Response (IntoResponse impl)

// DomainError variants map to:
//   InvalidField        -> UsecaseError::Validation (catch-all for unmatched)
//   BusinessRuleViolation -> UsecaseError::Validation (catch-all)
//   NotFound             -> UsecaseError::NotFound
//   Conflict             -> UsecaseError::Conflict
//   TierLimitExceeded    -> UsecaseError::TierLimitExceeded
//   RateLimitExceeded    -> UsecaseError::RateLimited

// RepoError variants map to:
//   NotFound            -> UsecaseError::NotFound
//   UniqueViolation     -> UsecaseError::Conflict
//   Db/DbWithEntity/etc -> UsecaseError::Infra (wraps in anyhow)

// UsecaseError variants map to HTTP:
//   NotFound            -> 404 NOT_FOUND
//   Validation          -> 400 VALIDATION_ERROR
//   Conflict            -> 409 CONFLICT
//   TierLimitExceeded   -> 409 LIMIT_REACHED (with upgrade_url)
//   RateLimited         -> 429 RATE_LIMITED  (with rate limit headers)
//   Gone                -> 410 GONE
//   Infra               -> 500 INTERNAL_ERROR (logs full error, returns generic message)

// In practice, usecases just use ? and the From impls handle everything:
//
// pub async fn execute(&self, input: Input) -> Result<Output, UsecaseError> {
//     let name = EndpointName::new(input.name)?;      // DomainError -> UsecaseError::Validation
//     let sub = self.sub_repo.find_by_user(&id).await?; // RepoError -> UsecaseError
//     // ...
// }
//
// And handlers just use ? with ApiError:
//
// pub async fn handler(...) -> Result<impl IntoResponse, ApiError> {
//     let output = usecase.execute(input).await?;      // UsecaseError -> ApiError
//     Ok((StatusCode::OK, Json(response)))
// }
