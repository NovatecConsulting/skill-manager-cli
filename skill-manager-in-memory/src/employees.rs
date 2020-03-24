use crate::{projects::ProjectDb, skills::SkillDb};
use serde::{Deserialize, Serialize};
use skill_manager::{
    employees::{
        usecase::{
            AddEmployee, AssignProjectToEmployee, AssignProjectToEmployeeError,
            AssignSkillToEmployee, AssignSkillToEmployeeError, DeleteEmployeeById, GetEmployeeById,
            ProjectAssignmentRequest, SkillAssignment,
        },
        Employee, EmployeeId, FirstName, Knowledge, LastName, ProjectAssignment,
        ProjectAssignmentId, SkillLevel,
    },
    skills::SkillId,
};
use std::collections::{BTreeMap, HashMap};
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize)]
pub struct EmployeeDb(pub HashMap<EmployeeId, Employee>);

impl AddEmployee for EmployeeDb {
    fn add(
        &mut self,
        first_name: FirstName,
        last_name: LastName,
    ) -> skill_manager::Result<Employee> {
        let id = EmployeeId(Uuid::new_v4());
        let employee = Employee {
            id: id.clone(),
            first_name,
            last_name,
            skills: BTreeMap::default(),
            projects: Vec::default(),
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

impl AssignProjectToEmployee for EmployeeDbWith<'_, ProjectDb> {
    fn assign_project(
        &mut self,
        employee_id: EmployeeId,
        project_assignment: ProjectAssignmentRequest,
    ) -> Result<ProjectAssignment, AssignProjectToEmployeeError> {
        let employee = self
            .employee_db
            .0
            .get_mut(&employee_id)
            .ok_or(AssignProjectToEmployeeError::EmployeeNotFound)?;
        let project = self
            .other_db
            .0
            .get(&project_assignment.project_id)
            .ok_or(AssignProjectToEmployeeError::ProjectNotFound)?;
        let project_assignment = ProjectAssignment {
            id: ProjectAssignmentId(Uuid::new_v4()),
            project: project.clone(),
            contribution: project_assignment.contribution,
            start_date: project_assignment.start_date,
            end_date: project_assignment.end_date,
        };
        employee.projects.push(project_assignment.clone());
        Ok(project_assignment)
    }
}

impl AssignSkillToEmployee for EmployeeDbWith<'_, SkillDb> {
    fn assign_skill(
        &mut self,
        employee_id: EmployeeId,
        skill_id: SkillId,
        skill_level: SkillLevel,
    ) -> Result<SkillAssignment, AssignSkillToEmployeeError> {
        let employee = self
            .employee_db
            .0
            .get_mut(&employee_id)
            .ok_or(AssignSkillToEmployeeError::EmployeeNotFound)?;

        let skill = self
            .other_db
            .0
            .get(&skill_id)
            .ok_or(AssignSkillToEmployeeError::SkillNotFound)?;

        employee.skills.insert(
            skill.label.clone(),
            Knowledge {
                level: skill_level.clone(),
            },
        );

        Ok(SkillAssignment {
            label: skill.label.clone(),
            level: skill_level,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use skill_manager::{
        employees::ProjectContribution,
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

    #[test]
    fn employee_api_test() -> anyhow::Result<()> {
        let mut employee_db = EmployeeDb::default();

        let employee = employee_db.add(first_name(), last_name())?;

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
        let employee = employee_db.add(first_name(), last_name())?;
        let skill_level = SkillLevel(5);

        let assignment = employee_db.with(&skill_db).assign_skill(
            employee.id.clone(),
            skill.id.clone(),
            skill_level.clone(),
        )?;

        assert_eq!(assignment.level, skill_level);

        assert_eq!(
            employee_db
                .get(employee.id.clone())?
                .unwrap()
                .skills
                .get(&skill_label())
                .cloned(),
            Some(Knowledge { level: skill_level })
        );

        Ok(())
    }

    #[test]
    fn assign_project_to_employee_test() -> anyhow::Result<()> {
        let mut project_db = ProjectDb::default();
        let mut employee_db = EmployeeDb::default();

        let project = project_db.add(project_label(), project_description())?;
        let employee = employee_db.add(first_name(), last_name())?;

        let project_assignment = employee_db.with(&project_db).assign_project(
            employee.id.clone(),
            ProjectAssignmentRequest {
                project_id: project.id.clone(),
                contribution: ProjectContribution("contribution".into()),
                start_date: Date::parse("2014-04-01", "%F").unwrap(),
                end_date: None,
            },
        )?;

        assert_eq!(
            employee_db.get(employee.id.clone())?.unwrap().projects,
            vec![project_assignment]
        );

        Ok(())
    }
}
