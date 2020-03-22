use crate::{
    skills::{Skill, SkillId, SkillLabel},
    Result,
};
use core::str::FromStr;

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

#[derive(Clone, Copy)]
pub struct PageNumber(pub usize);
impl FromStr for PageNumber {
    type Err = <usize as FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let inner = s.parse::<usize>()?;
        Ok(Self(inner))
    }
}

#[derive(Clone, Copy)]
pub struct PageSize(pub usize);
impl FromStr for PageSize {
    type Err = <usize as FromStr>::Err;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let inner = s.parse::<usize>()?;
        Ok(Self(inner))
    }
}

pub trait FindSkills {
    fn find(
        &self,
        page_number: Option<PageNumber>,
        page_size: Option<PageSize>,
    ) -> Result<Vec<Skill>>;
}

impl<F> FindSkills for F
where
    F: Fn(Option<PageNumber>, Option<PageSize>) -> Result<Vec<Skill>>,
{
    fn find(
        &self,
        page_number: Option<PageNumber>,
        page_size: Option<PageSize>,
    ) -> Result<Vec<Skill>> {
        self(page_number, page_size)
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
