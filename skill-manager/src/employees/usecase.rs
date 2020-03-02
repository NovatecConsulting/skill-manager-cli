use time::Date;

use crate::{
    employees::{
        Employee, EmployeeId, FirstName, LastName, ProjectAssignment, ProjectContribution,
        SkillLevel,
    },
    projects::ProjectId,
    skills::{SkillId, SkillLabel},
};

pub trait AddEmployee = Fn(FirstName, LastName) -> Result<Employee, String>;

pub trait DeleteEmployeeById = Fn(EmployeeId) -> Result<(), String>;

pub trait GetEmployeeById = Fn(EmployeeId) -> Result<Option<Employee>, String>;

pub struct ProjectAssignmentRequest {
    project_id: ProjectId,
    contribution: ProjectContribution,
    start_date: Date,
    end_date: Option<Date>,
}
pub trait AssignProjectToEmployee =
    Fn(EmployeeId, ProjectAssignmentRequest) -> Result<ProjectAssignment, String>;

pub struct SkillAssignment {
    label: SkillLabel,
    level: SkillLevel,
}
pub trait AssignSkillToEmployee =
    Fn(EmployeeId, SkillId, SkillLevel) -> Result<SkillAssignment, String>;

#[cfg(test)]
mod in_memory {
    use std::{
        cell::RefCell,
        collections::{BTreeMap, HashMap},
        rc::Rc,
    };
    use uuid::Uuid;

    use super::*;

    type EmployeeDb = Rc<RefCell<HashMap<EmployeeId, Employee>>>;

    pub struct EmployeeApi {
        add: Box<dyn AddEmployee>,
        delete: Box<dyn DeleteEmployeeById>,
        get: Box<dyn GetEmployeeById>,
        assign_project: Box<dyn AssignProjectToEmployee>,
        assign_skill: Box<dyn AssignSkillToEmployee>,
    }

    fn add(db: EmployeeDb) -> Box<dyn AddEmployee> {
        Box::new(move |first_name, last_name| {
            let id = EmployeeId(Uuid::new_v4());
            let employee = Employee {
                id: id.clone(),
                first_name,
                last_name,
                skills: BTreeMap::default(),
                projects: Vec::default(),
            };
            db.borrow_mut().insert(id, employee.clone());
            Ok(employee)
        })
    }
}
