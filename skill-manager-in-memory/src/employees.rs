use crate::{projects::ProjectDb, skills::SkillDb};
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
use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap},
    rc::Rc,
};
use uuid::Uuid;

pub type EmployeeDb = Rc<RefCell<EmployeeStore>>;
pub type EmployeeStore = HashMap<EmployeeId, Employee>;

pub fn employees_api(project_db: ProjectDb, skill_db: SkillDb) -> EmployeeApi {
    let employee_db = Rc::new(RefCell::new(HashMap::new()));
    employees_api_using(employee_db, project_db, skill_db)
}

pub fn employees_api_using(
    employee_db: EmployeeDb,
    project_db: ProjectDb,
    skill_db: SkillDb,
) -> EmployeeApi {
    EmployeeApi {
        employee_db,
        project_db,
        skill_db,
    }
}

pub struct EmployeeApi {
    employee_db: EmployeeDb,
    project_db: ProjectDb,
    skill_db: SkillDb,
}

impl AddEmployee for EmployeeApi {
    fn add(&self, first_name: FirstName, last_name: LastName) -> skill_manager::Result<Employee> {
        let id = EmployeeId(Uuid::new_v4());
        let employee = Employee {
            id: id.clone(),
            first_name,
            last_name,
            skills: BTreeMap::default(),
            projects: Vec::default(),
        };
        self.employee_db.borrow_mut().insert(id, employee.clone());
        Ok(employee)
    }
}

impl DeleteEmployeeById for EmployeeApi {
    fn delete(&self, employee_id: EmployeeId) -> skill_manager::Result<()> {
        let _ = self.employee_db.borrow_mut().remove(&employee_id);
        Ok(())
    }
}

impl GetEmployeeById for EmployeeApi {
    fn get(&self, employee_id: EmployeeId) -> skill_manager::Result<Option<Employee>> {
        Ok(self.employee_db.borrow().get(&employee_id).cloned())
    }
}

impl AssignProjectToEmployee for EmployeeApi {
    fn assign_project(
        &self,
        employee_id: EmployeeId,
        project_assignment: ProjectAssignmentRequest,
    ) -> std::result::Result<ProjectAssignment, AssignProjectToEmployeeError> {
        let mut employee_db = self.employee_db.borrow_mut();
        let project_db = self.project_db.borrow();
        let employee = employee_db
            .get_mut(&employee_id)
            .ok_or(AssignProjectToEmployeeError::EmployeeNotFound)?;
        let project = project_db
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

impl AssignSkillToEmployee for EmployeeApi {
    fn assign_skill(
        &self,
        employee_id: EmployeeId,
        skill_id: SkillId,
        skill_level: SkillLevel,
    ) -> Result<SkillAssignment, AssignSkillToEmployeeError> {
        let mut employee_db = self.employee_db.borrow_mut();
        let employee = employee_db
            .get_mut(&employee_id)
            .ok_or(AssignSkillToEmployeeError::EmployeeNotFound)?;

        let skill_db = self.skill_db.borrow();
        let skill = skill_db
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
