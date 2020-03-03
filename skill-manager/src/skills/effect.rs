use crate::{skills::Skill, Result};

pub trait AddSkill = Fn(Skill) -> Result<Skill>;
