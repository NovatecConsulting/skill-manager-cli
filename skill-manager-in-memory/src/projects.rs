use skill_manager::projects::{
    usecase::{AddProject, DeleteProject, GetProject},
    Project, ProjectId,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use uuid::Uuid;

pub fn projects_api() -> ProjectsApi {
    let db = Rc::new(RefCell::new(HashMap::new()));
    projects_api_using(db)
}

pub fn projects_api_using(db: ProjectDb) -> ProjectsApi {
    let add = add(db.clone());
    let delete = delete(db.clone());
    let get = get(db.clone());
    ProjectsApi {
        db,
        add,
        delete,
        get,
    }
}

pub struct ProjectsApi {
    pub db: ProjectDb,
    pub add: Box<dyn AddProject>,
    pub delete: Box<dyn DeleteProject>,
    pub get: Box<dyn GetProject>,
}

pub type ProjectDb = Rc<RefCell<ProjectStore>>;
pub type ProjectStore = HashMap<ProjectId, Project>;

fn add(db: ProjectDb) -> Box<dyn AddProject> {
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

fn delete(db: ProjectDb) -> Box<dyn DeleteProject> {
    Box::new(move |id| {
        let _ = db.borrow_mut().remove(&id);
        Ok(())
    })
}

fn get(db: ProjectDb) -> Box<dyn GetProject> {
    Box::new(move |id| Ok(db.borrow().get(&id).cloned()))
}

#[cfg(test)]
mod test {
    use super::*;
    use skill_manager::projects::{ProjectDescription, ProjectLabel};

    #[test]
    fn projects_api_test() -> skill_manager::Result<()> {
        let api = projects_api();
        let project = ProjectLabel("Example project".into());
        let added = api
            .add
            .add(project.clone(), ProjectDescription("".into()))?;
        assert_eq!(project, added.label);
        assert_eq!(api.get.get(added.id.clone())?, Some(added.clone()));
        api.delete.delete(added.id.clone())?;
        assert_eq!(api.get.get(added.id)?, None);

        Ok(())
    }
}
