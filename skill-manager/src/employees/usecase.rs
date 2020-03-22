use crate::{
    employees::{
        Employee, EmployeeId, FirstName, LastName, ProjectAssignment, ProjectContribution,
        SkillLevel,
    },
    projects::ProjectId,
    skills::{SkillId, SkillLabel},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::Date;

pub trait AddEmployee {
    fn add(&mut self, first_name: FirstName, last_name: LastName) -> crate::Result<Employee>;
}

impl<F> AddEmployee for F
where
    F: Fn(FirstName, LastName) -> crate::Result<Employee>,
{
    fn add(&mut self, first_name: FirstName, last_name: LastName) -> crate::Result<Employee> {
        self(first_name, last_name)
    }
}

pub trait DeleteEmployeeById {
    fn delete(&mut self, employee_id: EmployeeId) -> crate::Result<()>;
}

impl<F> DeleteEmployeeById for F
where
    F: Fn(EmployeeId) -> crate::Result<()>,
{
    fn delete(&mut self, employee_id: EmployeeId) -> crate::Result<()> {
        self(employee_id)
    }
}

pub trait GetEmployeeById {
    fn get(&self, employee_id: EmployeeId) -> crate::Result<Option<Employee>>;
}

impl<F> GetEmployeeById for F
where
    F: Fn(EmployeeId) -> crate::Result<Option<Employee>>,
{
    fn get(&self, employee_id: EmployeeId) -> crate::Result<Option<Employee>> {
        self(employee_id)
    }
}

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

pub trait AssignProjectToEmployee {
    fn assign_project(
        &mut self,
        employee_id: EmployeeId,
        project_assignment: ProjectAssignmentRequest,
    ) -> Result<ProjectAssignment, AssignProjectToEmployeeError>;
}

impl<F> AssignProjectToEmployee for F
where
    F: FnMut(
        EmployeeId,
        ProjectAssignmentRequest,
    ) -> Result<ProjectAssignment, AssignProjectToEmployeeError>,
{
    fn assign_project(
        &mut self,
        employee_id: EmployeeId,
        project_assignment: ProjectAssignmentRequest,
    ) -> Result<ProjectAssignment, AssignProjectToEmployeeError> {
        self(employee_id, project_assignment)
    }
}

#[derive(Deserialize, Serialize)]
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

pub trait AssignSkillToEmployee {
    fn assign_skill(
        &mut self,
        employee_id: EmployeeId,
        skill_id: SkillId,
        skill_level: SkillLevel,
    ) -> Result<SkillAssignment, AssignSkillToEmployeeError>;
}

impl<F> AssignSkillToEmployee for F
where
    F: FnMut(
        EmployeeId,
        SkillId,
        SkillLevel,
    ) -> Result<SkillAssignment, AssignSkillToEmployeeError>,
{
    fn assign_skill(
        &mut self,
        employee_id: EmployeeId,
        skill_id: SkillId,
        skill_level: SkillLevel,
    ) -> Result<SkillAssignment, AssignSkillToEmployeeError> {
        self(employee_id, skill_id, skill_level)
    }
}
