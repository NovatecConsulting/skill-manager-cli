use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod effect;
pub mod usecase;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct Skill {
    pub id: SkillId,
    pub label: SkillLabel,
}

gen_wrapper!(SkillId: Uuid, SkillLabel: String);
