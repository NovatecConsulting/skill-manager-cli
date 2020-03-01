pub mod bl;
pub mod effect;

use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct SkillLabel(String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Skill {
    id: Uuid,
    label: SkillLabel,
}

#[cfg(test)]
mod test {}
