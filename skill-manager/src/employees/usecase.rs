use super::SkillAssignment;
use crate::{
    employees::{
        EmailAddress, Employee, EmployeeId, FirstName, LastName, ProjectAssignment,
        ProjectContribution, SkillLevel, TelephoneNumber, Title,
    },
    projects::ProjectId,
    skills::SkillId,
};
use thiserror::Error;
use time::Date;

pub struct AddEmployeeRequest {
    pub first_name: FirstName,
    pub last_name: LastName,
    pub title: Title,
    pub email: EmailAddress,
    pub telephone: TelephoneNumber,
}

pub trait AddEmployee {
    fn add(&mut self, request: AddEmployeeRequest) -> crate::Result<Employee>;
}

impl<F> AddEmployee for F
where
    F: Fn(AddEmployeeRequest) -> crate::Result<Employee>,
{
    fn add(&mut self, request: AddEmployeeRequest) -> crate::Result<Employee> {
        self(request)
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

pub trait FindEmployees {
    fn find_employees(&self) -> crate::Result<Vec<Employee>>;
}

impl<F> FindEmployees for F
where
    F: Fn() -> crate::Result<Vec<Employee>>,
{
    fn find_employees(&self) -> crate::Result<Vec<Employee>> {
        self()
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
    pub employee_id: EmployeeId,
    pub project_id: ProjectId,
    pub contribution: ProjectContribution,
    pub start_date: Date,
    pub end_date: Option<Date>,
}

#[derive(Error, Debug)]
pub enum AssignProjectToEmployeeError {
    #[error(transparent)]
    EmployeeNotFound(#[from] EmployeeNotFoundError),
    #[error(transparent)]
    ProjectNotFound(#[from] ProjectNotFoundError),
}

#[error("Employee not found")]
#[derive(Error, Debug)]
pub struct EmployeeNotFoundError;

#[error("Project not found")]
#[derive(Error, Debug)]
pub struct ProjectNotFoundError;

pub trait CreateProjectAssignment {
    fn create_project_assignment(
        &mut self,
        project_assignment: ProjectAssignmentRequest,
    ) -> Result<ProjectAssignment, AssignProjectToEmployeeError>;
}

impl<F> CreateProjectAssignment for F
where
    F: FnMut(ProjectAssignmentRequest) -> Result<ProjectAssignment, AssignProjectToEmployeeError>,
{
    fn create_project_assignment(
        &mut self,
        project_assignment: ProjectAssignmentRequest,
    ) -> Result<ProjectAssignment, AssignProjectToEmployeeError> {
        self(project_assignment)
    }
}

#[derive(Error, Debug)]
pub enum DeleteProjectAssignmentError {
    #[error(transparent)]
    EmployeeNotFound(#[from] EmployeeNotFoundError),
    #[error(transparent)]
    ProjectNotFound(#[from] ProjectNotFoundError),
}

pub trait DeleteProjectAssignment {
    fn delete_project_assignment(
        &mut self,
        employee_id: EmployeeId,
        assignment_id: String,
    ) -> Result<(), DeleteProjectAssignmentError>;
}

impl<F> DeleteProjectAssignment for F
where
    F: FnMut(EmployeeId, String) -> Result<(), DeleteProjectAssignmentError>,
{
    fn delete_project_assignment(
        &mut self,
        employee_id: EmployeeId,
        assignment_id: String,
    ) -> Result<(), DeleteProjectAssignmentError> {
        self(employee_id, assignment_id)
    }
}

#[derive(Error, Debug)]
pub enum AssignSkillToEmployeeError {
    #[error("Employee not found")]
    EmployeeNotFound,
    #[error("Skill not found")]
    SkillNotFound,
}

pub struct SetSkillKnowledgeRequest {
    pub employee_id: EmployeeId,
    pub skill_id: SkillId,
    pub level: SkillLevel,
    pub secret: bool,
}

pub trait SetSkillKnowledgeOfEmployee {
    fn set_skill_knowledge_of_employee(
        &mut self,
        request: SetSkillKnowledgeRequest,
    ) -> Result<SkillAssignment, AssignSkillToEmployeeError>;
}

impl<F> SetSkillKnowledgeOfEmployee for F
where
    F: FnMut(SetSkillKnowledgeRequest) -> Result<SkillAssignment, AssignSkillToEmployeeError>,
{
    fn set_skill_knowledge_of_employee(
        &mut self,
        request: SetSkillKnowledgeRequest,
    ) -> Result<SkillAssignment, AssignSkillToEmployeeError> {
        self(request)
    }
}

pub trait DeleteSkillKnowledgeOfEmployee {
    fn delete_skill_knowledge_of_employee(
        &mut self,
        employee_id: EmployeeId,
        skill_id: SkillId,
    ) -> crate::Result<()>;
}

impl<F> DeleteSkillKnowledgeOfEmployee for F
where
    F: FnMut(EmployeeId, SkillId) -> crate::Result<()>,
{
    fn delete_skill_knowledge_of_employee(
        &mut self,
        employee_id: EmployeeId,
        skill_id: SkillId,
    ) -> crate::Result<()> {
        self(employee_id, skill_id)
    }
}
