use crate::{
    employees::in_memory::{employees_api, employees_api_using, EmployeeApi, EmployeeDb},
    projects::in_memory::{projects_api, projects_api_using, ProjectDb, ProjectsApi},
    skills::in_memory::{skills_api, skills_api_using, SkillDb, SkillsApi},
};

pub struct InMemoryApi {
    pub skills: SkillsApi,
    pub projects: ProjectsApi,
    pub employees: EmployeeApi,
}

pub fn in_memory_api() -> InMemoryApi {
    let skills = skills_api();
    let projects = projects_api();
    let employees = employees_api(projects.db.clone(), skills.db.clone());
    InMemoryApi {
        skills,
        projects,
        employees,
    }
}

pub fn in_memory_api_using(
    skill_db: SkillDb,
    project_db: ProjectDb,
    employee_db: EmployeeDb,
) -> InMemoryApi {
    let skills = skills_api_using(skill_db.into());
    let projects = projects_api_using(project_db.into());
    let employees = employees_api_using(employee_db.into(), projects.db.clone(), skills.db.clone());
    InMemoryApi {
        skills,
        projects,
        employees,
    }
}
