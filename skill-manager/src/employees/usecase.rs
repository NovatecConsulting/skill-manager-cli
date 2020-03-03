use thiserror::Error;
use time::Date;

use crate::{
    employees::{
        Employee, EmployeeId, FirstName, LastName, ProjectAssignment, ProjectContribution,
        SkillLevel,
    },
    projects::ProjectId,
    skills::{SkillId, SkillLabel},
};

pub trait AddEmployee = Fn(FirstName, LastName) -> crate::Result<Employee>;

pub trait DeleteEmployeeById = Fn(EmployeeId) -> crate::Result<()>;

pub trait GetEmployeeById = Fn(EmployeeId) -> crate::Result<Option<Employee>>;

pub struct ProjectAssignmentRequest {
    pub project_id: ProjectId,
    pub contribution: ProjectContribution,
    pub start_date: Date,
    pub end_date: Option<Date>,
}

#[derive(Error, Debug)]
pub enum AssignProjectToEmployeeError {
    #[error("Employee not found")]
    EmployeeNotFound,
    #[error("Project not found")]
    ProjectNotFound,
}

pub trait AssignProjectToEmployee = Fn(
    EmployeeId,
    ProjectAssignmentRequest,
) -> Result<ProjectAssignment, AssignProjectToEmployeeError>;

pub struct SkillAssignment {
    pub label: SkillLabel,
    pub level: SkillLevel,
}
#[derive(Error, Debug)]
pub enum AssignSkillToEmployeeError {
    #[error("Employee not found")]
    EmployeeNotFound,
    #[error("Skill not found")]
    SkillNotFound,
}

pub trait AssignSkillToEmployee =
    Fn(EmployeeId, SkillId, SkillLevel) -> Result<SkillAssignment, AssignSkillToEmployeeError>;
