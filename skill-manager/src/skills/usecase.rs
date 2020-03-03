use crate::{
    skills::{Skill, SkillId, SkillLabel},
    Result,
};
use core::str::FromStr;

pub trait AddSkill = Fn(SkillLabel) -> Result<Skill>;

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

pub trait FindSkills = Fn(Option<PageNumber>, Option<PageSize>) -> Result<Vec<Skill>>;

pub trait GetSkillById = Fn(SkillId) -> Result<Option<Skill>>;

pub trait DeleteSkillById = Fn(SkillId) -> Result<()>;

pub mod in_memory {}
