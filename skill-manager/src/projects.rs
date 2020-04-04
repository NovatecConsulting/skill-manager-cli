use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod usecase;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct Project {
    pub id: ProjectId,
    pub label: ProjectLabel,
    pub description: ProjectDescription,
}

gen_wrapper!(
    ProjectId: Uuid,
    ProjectLabel: String,
    ProjectDescription: String
);
