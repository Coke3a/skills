// Template destination:
// src/usecases/<feature>/<action>_<entity>_test.rs
//
// Parent module must include:
// #[cfg(test)]
// mod <action>_<entity>_test;
//
// Use this for usecase orchestration and error semantics.
// Do not use this template to prove Diesel/Postgres behavior.

use super::create_example_entity::*;
use crate::domain::entities::example_entity::ExampleEntity;
use crate::domain::repositories::example_repository::ExampleRepository;
use crate::repositories::errors::RepoError;
use crate::usecases::errors::UsecaseError;

#[derive(Default)]
struct FakeExampleRepository {
    saved: Vec<ExampleEntity>,
    next_error: Option<RepoError>,
}

#[async_trait::async_trait]
impl ExampleRepository for FakeExampleRepository {
    async fn save(&mut self, entity: ExampleEntity) -> Result<(), RepoError> {
        if let Some(error) = self.next_error.take() {
            return Err(error);
        }

        self.saved.push(entity);
        Ok(())
    }
}

#[tokio::test]
async fn creates_entity_when_input_is_valid() {
    let repo = FakeExampleRepository::default();
    let usecase = CreateExampleEntityUseCase::new(repo);
    let input = CreateExampleEntityInput {
        name: "Valid name".to_string(),
    };

    let result = usecase.execute(input).await;

    assert!(matches!(result, Ok(CreateExampleEntityOutput { .. })));
}

#[tokio::test]
async fn returns_validation_error_when_name_is_empty() {
    let repo = FakeExampleRepository::default();
    let usecase = CreateExampleEntityUseCase::new(repo);
    let input = CreateExampleEntityInput {
        name: "".to_string(),
    };

    let result = usecase.execute(input).await;

    assert!(matches!(result, Err(UsecaseError::Validation(_))));
}

#[tokio::test]
async fn returns_conflict_when_repository_reports_duplicate() {
    let repo = FakeExampleRepository {
        next_error: Some(RepoError::Conflict("duplicate".into())),
        ..Default::default()
    };
    let usecase = CreateExampleEntityUseCase::new(repo);
    let input = CreateExampleEntityInput {
        name: "Valid name".to_string(),
    };

    let result = usecase.execute(input).await;

    assert!(matches!(result, Err(UsecaseError::Conflict(_))));
}

#[tokio::test]
async fn maps_repo_error_to_usecase_error() {
    let repo = FakeExampleRepository {
        next_error: Some(RepoError::Unavailable("database unavailable".into())),
        ..Default::default()
    };
    let usecase = CreateExampleEntityUseCase::new(repo);
    let input = CreateExampleEntityInput {
        name: "Valid name".to_string(),
    };

    let result = usecase.execute(input).await;

    assert!(matches!(result, Err(UsecaseError::Repository(_))));
}
