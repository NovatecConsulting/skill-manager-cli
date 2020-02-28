use crate::skills::Skill;
use crate::skills::SkillLabel;

pub trait AddSkill {
    fn add_skill(label: SkillLabel) -> Result<Skill, String>;
}
