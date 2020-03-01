use uuid::Uuid;

use crate::skills::{effect, Skill, SkillLabel};

pub trait IdGenerator = Fn() -> Uuid;

pub trait AddSkill = Fn(SkillLabel) -> Result<Skill, String>;

pub fn add_skill<I, S>(id_generator: I, persist_skill: S) -> impl AddSkill
where
    I: IdGenerator,
    S: effect::AddSkill,
{
    move |label| {
        let skill = Skill {
            id: id_generator(),
            label,
        };
        let persisted = persist_skill(skill)?;
        Ok(persisted)
    }
}

pub struct PageNumber(usize);
pub struct PageSize(usize);
pub trait FindSkills = Fn(PageNumber, PageSize) -> Result<Vec<Skill>, String>;

pub trait GetSkillById = Fn(Uuid) -> Option<Skill>;
