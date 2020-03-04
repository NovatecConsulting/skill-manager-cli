use crate::{
    employees::{
        usecase::{
            AddEmployee, AssignProjectToEmployee, AssignProjectToEmployeeError,
            AssignSkillToEmployee, AssignSkillToEmployeeError, DeleteEmployeeById, GetEmployeeById,
            ProjectAssignmentRequest, SkillAssignment,
        },
        Employee, EmployeeId, Knowledge, ProjectAssignment, ProjectAssignmentId,
    },
    projects::in_memory::ProjectDb,
    skills::in_memory::SkillDb,
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
    let add = add(employee_db.clone());
    let delete = delete(employee_db.clone());
    let get = get(employee_db.clone());
    let assign_project = assign_project(employee_db.clone(), project_db);
    let assign_skill = assign_skill(employee_db, skill_db);
    EmployeeApi {
        add,
        delete,
        get,
        assign_project,
        assign_skill,
    }
}

pub struct EmployeeApi {
    pub add: Box<dyn AddEmployee>,
    pub delete: Box<dyn DeleteEmployeeById>,
    pub get: Box<dyn GetEmployeeById>,
    pub assign_project: Box<dyn AssignProjectToEmployee>,
    pub assign_skill: Box<dyn AssignSkillToEmployee>,
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

fn delete(db: EmployeeDb) -> Box<dyn DeleteEmployeeById> {
    Box::new(move |employee_id| {
        let _ = db.borrow_mut().remove(&employee_id);
        Ok(())
    })
}

fn get(db: EmployeeDb) -> Box<dyn GetEmployeeById> {
    Box::new(move |employee_id| Ok(db.borrow().get(&employee_id).cloned()))
}

fn assign_project(
    employee_db: EmployeeDb,
    project_db: ProjectDb,
) -> Box<dyn AssignProjectToEmployee> {
    Box::new(
        move |employee_id, project_assignment: ProjectAssignmentRequest| {
            let mut employee_db = employee_db.borrow_mut();
            let project_db = project_db.borrow();
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
        },
    )
}

fn assign_skill(employee_db: EmployeeDb, skill_db: SkillDb) -> Box<dyn AssignSkillToEmployee> {
    Box::new(move |employee_id, skill_id, level| {
        let mut employee_db = employee_db.borrow_mut();
        let employee = employee_db
            .get_mut(&employee_id)
            .ok_or(AssignSkillToEmployeeError::EmployeeNotFound)?;

        let skill_db = skill_db.borrow();
        let skill = skill_db
            .get(&skill_id)
            .ok_or(AssignSkillToEmployeeError::SkillNotFound)?;

        employee.skills.insert(
            skill.label.clone(),
            Knowledge {
                level: level.clone(),
            },
        );

        Ok(SkillAssignment {
            label: skill.label.clone(),
            level,
        })
    })
}
