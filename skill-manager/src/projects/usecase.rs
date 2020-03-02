use crate::projects::{Project, ProjectDescription, ProjectId, ProjectLabel};

pub trait AddProject = Fn(ProjectLabel, ProjectDescription) -> Result<Project, String>;

pub trait DeleteProject = Fn(ProjectId) -> Result<(), String>;

pub trait GetProject = Fn(ProjectId) -> Result<Option<Project>, String>;

#[cfg(test)]
mod in_memory {
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    use uuid::Uuid;

    use super::*;

    pub fn api() -> ProjectsApi {
        let db = Rc::new(RefCell::new(HashMap::new()));
        let add = add(db.clone());
        let delete = delete(db.clone());
        let get = get(db);
        ProjectsApi { add, delete, get }
    }

    #[test]
    fn projects_api_test() -> Result<(), String> {
        let api = api();
        let project = ProjectLabel("Example project".into());
        let added = (api.add)(project.clone(), ProjectDescription("".into()))?;
        assert_eq!(project, added.label);
        assert_eq!((api.get)(added.id.clone())?, Some(added.clone()));
        let _ = (api.delete)(added.id.clone())?;
        assert_eq!((api.get)(added.id.clone())?, None);

        Ok(())
    }

    pub struct ProjectsApi {
        pub add: Box<dyn AddProject>,
        pub delete: Box<dyn DeleteProject>,
        pub get: Box<dyn GetProject>,
    }

    type Db = Rc<RefCell<HashMap<ProjectId, Project>>>;

    fn add(db: Db) -> Box<dyn AddProject> {
        Box::new(move |label, description| {
            let id = ProjectId(Uuid::new_v4());
            let project = Project {
                id: id.clone(),
                label,
                description,
            };
            db.borrow_mut().insert(id, project.clone());
            Ok(project)
        })
    }

    fn delete(db: Db) -> Box<dyn DeleteProject> {
        Box::new(move |id| {
            let _ = db.borrow_mut().remove(&id);
            Ok(())
        })
    }

    fn get(db: Db) -> Box<dyn GetProject> {
        Box::new(move |id| Ok(db.borrow().get(&id).cloned()))
    }
}
