use crate::skills::Skill;

pub trait AddSkill = Fn(Skill) -> Result<Skill, String>;
