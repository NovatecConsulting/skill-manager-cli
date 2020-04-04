use crate::{
    projects::{Project, ProjectDescription, ProjectId, ProjectLabel},
    Result,
};

pub trait AddProject {
    fn add(
        &mut self,
        project_label: ProjectLabel,
        project_description: ProjectDescription,
    ) -> Result<Project>;
}

impl<F> AddProject for F
where
    F: Fn(ProjectLabel, ProjectDescription) -> Result<Project>,
{
    fn add(
        &mut self,
        project_label: ProjectLabel,
        project_description: ProjectDescription,
    ) -> Result<Project> {
        self(project_label, project_description)
    }
}

pub trait DeleteProject {
    fn delete(&mut self, project_id: ProjectId) -> Result<()>;
}

impl<F> DeleteProject for F
where
    F: Fn(ProjectId) -> Result<()>,
{
    fn delete(&mut self, project_id: ProjectId) -> Result<()> {
        self(project_id)
    }
}

pub trait FindProjects {
    fn find_projects(&self) -> Result<Vec<Project>>;
}

impl<F> FindProjects for F
where
    F: Fn() -> Result<Vec<Project>>,
{
    fn find_projects(&self) -> Result<Vec<Project>> {
        self()
    }
}

pub trait GetProject {
    fn get(&self, project_id: ProjectId) -> Result<Option<Project>>;
}

impl<F> GetProject for F
where
    F: Fn(ProjectId) -> Result<Option<Project>>,
{
    fn get(&self, project_id: ProjectId) -> Result<Option<Project>> {
        self(project_id)
    }
}
