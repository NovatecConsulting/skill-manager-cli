use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod effect;
pub mod in_memory;
pub mod usecase;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct Skill {
    pub id: SkillId,
    pub label: SkillLabel,
}

gen_wrapper!(SkillId: Uuid, SkillLabel: String);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_skill_returns_itself() {
        let api = in_memory::skills_api();
        let label_to_create = SkillLabel(String::from("Example skill"));
        let added = (api.add)(label_to_create.clone()).unwrap();
        assert_eq!(added.label, label_to_create);
    }
}
