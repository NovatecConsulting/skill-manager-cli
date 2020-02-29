pub mod bl;
pub mod http;
pub mod persist;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
// use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Clone)]
#[serde(transparent)]
pub struct SkillLabel(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Clone)]
pub struct Skill {
    id: Uuid,
    label: SkillLabel,
}

#[cfg(test)]
mod test {}
