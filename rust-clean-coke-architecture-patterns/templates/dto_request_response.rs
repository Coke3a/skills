use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{Project, ProjectStatus};

#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct ProjectResponse {
    pub id: Uuid,
    pub name: String,
    pub status: String,
}

impl From<Project> for ProjectResponse {
    fn from(project: Project) -> Self {
        Self {
            id: project.id.as_uuid(),
            name: project.name.as_str().to_string(),
            status: match project.status {
                ProjectStatus::Active => "active".to_string(),
                ProjectStatus::Archived => "archived".to_string(),
            },
        }
    }
}
