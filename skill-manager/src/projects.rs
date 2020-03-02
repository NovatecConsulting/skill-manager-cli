use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod usecase;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct Project {
    id: ProjectId,
    label: ProjectLabel,
    description: ProjectDescription,
}

gen_wrapper!(ProjectId: Uuid);
gen_wrapper!(ProjectLabel, ProjectDescription);
