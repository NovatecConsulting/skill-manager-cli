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
        persist_skill(&skill)?;
        Ok(skill)
    }
}
