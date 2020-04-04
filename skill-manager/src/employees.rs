use crate::{
    projects::{ProjectDescription, ProjectLabel},
    skills::SkillLabel,
};
use serde::{Deserialize, Serialize};
use time::{Date, OffsetDateTime};
use uuid::Uuid;

pub mod usecase;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct Employee {
    pub id: EmployeeId,
    pub first_name: FirstName,
    pub last_name: LastName,
    pub title: Title,
    pub email: EmailAddress,
    pub telephone: TelephoneNumber,
    pub skills: Vec<SkillAssignment>,
    pub projects: Vec<ProjectAssignment>,
    pub last_update: OffsetDateTime,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct SkillAssignment {
    pub label: SkillLabel,
    pub level: SkillLevel,
    pub secret: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Deserialize, Serialize, Hash)]
pub struct ProjectAssignment {
    pub label: ProjectLabel,
    pub description: ProjectDescription,
    pub contribution: ProjectContribution,
    pub start_date: Date,
    pub end_date: Option<Date>,
}

gen_wrapper!(
    EmployeeId: Uuid,
    FirstName: String,
    LastName: String,
    Title: String,
    EmailAddress: String,
    TelephoneNumber: String,
    ProjectAssignmentId: Uuid,
    ProjectContribution: String,
    SkillLevel: usize
);
