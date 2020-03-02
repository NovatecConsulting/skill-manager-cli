use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod effect;
pub mod usecase;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct Skill {
    // id: SkillId,
    label: SkillLabel,
}

gen_wrapper!(SkillId: Uuid, SkillLabel: String);

#[cfg(test)]
mod test {
    use super::{usecase::in_memory, *};

    #[test]
    fn add_skill_returns_itself() {
        let api = in_memory::api();
        let label_to_create = SkillLabel(String::from("Example skill"));
        let added = (api.add)(label_to_create.clone()).unwrap();
        assert_eq!(added.label, label_to_create);
    }
}
