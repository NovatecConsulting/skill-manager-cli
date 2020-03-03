use crate::{
    projects::{Project, ProjectDescription, ProjectId, ProjectLabel},
    Result,
};

pub trait AddProject = Fn(ProjectLabel, ProjectDescription) -> Result<Project>;

pub trait DeleteProject = Fn(ProjectId) -> Result<()>;

pub trait GetProject = Fn(ProjectId) -> Result<Option<Project>>;
