use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use time::Date;
use uuid::Uuid;

use crate::{projects::Project, skills::Skill};

pub mod usecase;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct Employee {
    pub id: EmployeeId,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub skills: BTreeMap<Skill, Knowledge>,
    pub projects: Vec<ProjectAssignment>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct ProjectAssignment {
    pub id: ProjectAssignmentId,
    pub project: Project,
    pub contribution: ProjectContribution,
    pub start_date: Date,
    pub end_date: Option<Date>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct Knowledge {
    level: SkillLevel,
}

gen_wrapper!(
    EmployeeId: Uuid,
    FirstName: String,
    LastName: String,
    ProjectAssignmentId: Uuid,
    ProjectContribution: String,
    SkillLevel: usize
);
