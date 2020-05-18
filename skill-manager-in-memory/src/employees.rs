use crate::{projects::ProjectDb, skills::SkillDb};
use serde::{Deserialize, Serialize};
use skill_manager::employees::{
    usecase::{
        AddEmployee, AddEmployeeRequest, AssignProjectToEmployeeError, AssignSkillToEmployeeError,
        CreateProjectAssignment, DeleteEmployeeById, EmployeeNotFoundError, FindEmployees,
        GetEmployeeById, ProjectAssignmentRequest, ProjectNotFoundError,
        SetSkillKnowledgeOfEmployee,
    },
    Employee, EmployeeId, ProjectAssignment, SkillAssignment,
};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize)]
pub struct EmployeeDb(pub HashMap<EmployeeId, Employee>);

impl AddEmployee for EmployeeDb {
    fn add(&mut self, request: AddEmployeeRequest) -> skill_manager::Result<Employee> {
        let AddEmployeeRequest {
            first_name,
            last_name,
            title,
            email,
            telephone,
        } = request;
        let id = EmployeeId(Uuid::new_v4());
        let employee = Employee {
            id: id.clone(),
            first_name,
            last_name,
            title,
            email,
            telephone,
            skills: vec![],
            projects: vec![],
            last_update: time::OffsetDateTime::now_utc(),
        };
        self.0.insert(id, employee.clone());
        Ok(employee)
    }
}

impl DeleteEmployeeById for EmployeeDb {
    fn delete(&mut self, employee_id: EmployeeId) -> skill_manager::Result<()> {
        let _ = self.0.remove(&employee_id);
        Ok(())
    }
}

impl GetEmployeeById for EmployeeDb {
    fn get(&self, employee_id: EmployeeId) -> skill_manager::Result<Option<Employee>> {
        Ok(self.0.get(&employee_id).cloned())
    }
}

impl FindEmployees for EmployeeDb {
    fn find_employees(&self) -> skill_manager::Result<Vec<Employee>> {
        Ok(self.0.values().cloned().collect())
    }
}

impl EmployeeDb {
    pub fn with<'a, Db>(&'a mut self, other_db: &'a Db) -> EmployeeDbWith<'a, Db> {
        EmployeeDbWith {
            employee_db: self,
            other_db,
        }
    }
}

pub struct EmployeeDbWith<'a, Db> {
    employee_db: &'a mut EmployeeDb,
    other_db: &'a Db,
}

impl CreateProjectAssignment for EmployeeDbWith<'_, ProjectDb> {
    fn create_project_assignment(
        &mut self,
        project_assignment: ProjectAssignmentRequest,
    ) -> Result<ProjectAssignment, AssignProjectToEmployeeError> {
        let employee = self
            .employee_db
            .0
            .get_mut(&project_assignment.employee_id)
            .ok_or(EmployeeNotFoundError)?;
        let project = self
            .other_db
            .0
            .get(&project_assignment.project_id)
            .ok_or(ProjectNotFoundError)?;
        let project_assignment = ProjectAssignment {
            label: project.label.clone(),
            description: project.description.clone(),
            contribution: project_assignment.contribution,
            start_date: project_assignment.start_date,
            end_date: project_assignment.end_date,
        };
        employee.projects.push(project_assignment.clone());
        Ok(project_assignment)
    }
}

impl SetSkillKnowledgeOfEmployee for EmployeeDbWith<'_, SkillDb> {
    fn set_skill_knowledge_of_employee(
        &mut self,
        request: skill_manager::employees::usecase::SetSkillKnowledgeRequest,
    ) -> Result<SkillAssignment, AssignSkillToEmployeeError> {
        let employee = self
            .employee_db
            .0
            .get_mut(&request.employee_id)
            .ok_or(AssignSkillToEmployeeError::EmployeeNotFound)?;

        let skill = self
            .other_db
            .0
            .get(&request.skill_id)
            .ok_or(AssignSkillToEmployeeError::SkillNotFound)?;

        let assignment = SkillAssignment {
            label: skill.label.clone(),
            level: request.level,
            secret: request.secret,
        };
        employee.skills.push(assignment.clone());

        Ok(assignment)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use skill_manager::{
        employees::{
            usecase::SetSkillKnowledgeRequest, EmailAddress, FirstName, LastName,
            ProjectContribution, SkillLevel, TelephoneNumber, Title,
        },
        projects::{usecase::AddProject, ProjectDescription, ProjectLabel},
        skills::{usecase::AddSkill, SkillLabel},
    };
    use time::Date;

    fn skill_label() -> SkillLabel {
        SkillLabel("test skill".into())
    }

    fn project_label() -> ProjectLabel {
        ProjectLabel("test project".into())
    }

    fn project_description() -> ProjectDescription {
        ProjectDescription("".into())
    }

    fn first_name() -> FirstName {
        FirstName("first name".into())
    }

    fn last_name() -> LastName {
        LastName("last name".into())
    }

    fn add_employee_request() -> AddEmployeeRequest {
        AddEmployeeRequest {
            first_name: first_name(),
            last_name: last_name(),
            title: Title("".into()),
            email: EmailAddress("".into()),
            telephone: TelephoneNumber("".into()),
        }
    }

    #[test]
    fn employee_api_test() -> anyhow::Result<()> {
        let mut employee_db = EmployeeDb::default();

        let employee = employee_db.add(add_employee_request())?;

        assert_eq!(
            employee_db.get(employee.id.clone())?,
            Some(employee.clone())
        );

        employee_db.delete(employee.id.clone())?;

        assert_eq!(employee_db.get(employee.id.clone())?, None);

        Ok(())
    }

    #[test]
    fn assign_skill_to_employee_test() -> anyhow::Result<()> {
        let mut skill_db = SkillDb::default();
        let mut employee_db = EmployeeDb::default();

        let skill = skill_db.add(skill_label())?;
        let employee = employee_db.add(add_employee_request())?;
        let skill_level = SkillLevel(5);

        let assignment = employee_db
            .with(&skill_db)
            .set_skill_knowledge_of_employee(SetSkillKnowledgeRequest {
                employee_id: employee.id.clone(),
                skill_id: skill.id.clone(),
                level: skill_level.clone(),
                secret: false,
            })?;

        assert_eq!(assignment.level, skill_level);

        assert_eq!(
            employee_db.get(employee.id.clone())?.unwrap().skills,
            vec![SkillAssignment {
                label: skill.label.clone(),
                level: skill_level,
                secret: false
            }]
        );

        Ok(())
    }

    #[test]
    fn assign_project_to_employee_test() -> anyhow::Result<()> {
        let mut project_db = ProjectDb::default();
        let mut employee_db = EmployeeDb::default();

        let project = project_db.add(project_label(), project_description())?;
        let employee = employee_db.add(AddEmployeeRequest {
            first_name: first_name(),
            last_name: last_name(),
            title: Title("".into()),
            email: EmailAddress("".into()),
            telephone: TelephoneNumber("".into()),
        })?;

        let project_assignment =
            employee_db
                .with(&project_db)
                .create_project_assignment(ProjectAssignmentRequest {
                    employee_id: employee.id.clone(),
                    project_id: project.id.clone(),
                    contribution: ProjectContribution("contribution".into()),
                    start_date: Date::parse("2014-04-01", "%F").unwrap(),
                    end_date: None,
                })?;

        assert_eq!(
            employee_db.get(employee.id.clone())?.unwrap().projects,
            vec![project_assignment]
        );

        Ok(())
    }
}
