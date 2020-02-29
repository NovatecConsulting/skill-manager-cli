use serde::Deserialize;
use uuid::Uuid;

use crate::skills::{persist::PersistSkill, Skill, SkillLabel};

pub trait IdGenerator = Fn() -> Uuid + Sync + Send + 'static;

pub trait AddSkill = Fn(SkillLabel) -> Result<Skill, String> + Sync + Send + 'static;

pub fn add_skill<I, S>(id_generator: I, persist_skill: S) -> impl AddSkill
where
    I: IdGenerator,
    S: PersistSkill,
{
    move |label| {
        let skill = Skill {
            id: id_generator(),
            label,
        };
        persist_skill(skill.clone())?;
        Ok(skill)
    }
}

#[derive(Deserialize)]
pub struct PageNumber(pub usize);
impl Default for PageNumber {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Deserialize)]
pub struct PageSize(pub usize);
impl Default for PageSize {
    fn default() -> Self {
        Self(100)
    }
}

pub trait FindSkills =
    Fn(PageNumber, PageSize) -> Result<Vec<Skill>, String> + Sync + Send + 'static;
