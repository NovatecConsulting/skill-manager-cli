use uuid::Uuid;

use crate::skills::Skill;
use crate::skills::SkillLabel;

pub trait IAddSkill {
    fn add_skill(label: SkillLabel) -> Result<Skill, String>;
}

trait IdGenerator {
    fn generate_id() -> Uuid;
}

trait PersistSkill {
    fn persist_skill(skill: &Skill) -> Result<(), String>;
}

struct AddSkill;

impl IAddSkill for AddSkill {
    fn add_skill<I: IdGenerator, S: PersistSkill>(label: SkillLabel) -> Result<Skill, String> {
        let skill = Skill {
            id: I::generate_id(),
            label,
        };
        S::persist_skill(&skill)?;
        Ok(skill)
    }
}
