mod bl;
mod http;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
// use serde_derive::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(transparent)]
pub struct SkillLabel(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
pub struct Skill {
    id: Uuid,
    label: SkillLabel,
}

#[cfg(test)]
mod test {}
