use crate::{
    skills::{Skill, SkillId, SkillLabel},
    Result,
};

pub trait AddSkill {
    fn add(&mut self, skill_label: SkillLabel) -> Result<Skill>;
}

impl<F> AddSkill for F
where
    F: FnMut(SkillLabel) -> Result<Skill>,
{
    fn add(&mut self, skill_label: SkillLabel) -> Result<Skill> {
        self(skill_label)
    }
}

pub trait FindSkills {
    fn find(&self) -> Result<Vec<Skill>>;
}

impl<F> FindSkills for F
where
    F: Fn() -> Result<Vec<Skill>>,
{
    fn find(&self) -> Result<Vec<Skill>> {
        self()
    }
}

pub trait GetSkillById {
    fn get(&self, skill_id: SkillId) -> Result<Option<Skill>>;
}

impl<F> GetSkillById for F
where
    F: Fn(SkillId) -> Result<Option<Skill>>,
{
    fn get(&self, skill_id: SkillId) -> Result<Option<Skill>> {
        self(skill_id)
    }
}

pub trait DeleteSkillById {
    fn delete(&mut self, skill_id: SkillId) -> Result<()>;
}

impl<F> DeleteSkillById for F
where
    F: FnMut(SkillId) -> Result<()>,
{
    fn delete(&mut self, skill_id: SkillId) -> Result<()> {
        self(skill_id)
    }
}
