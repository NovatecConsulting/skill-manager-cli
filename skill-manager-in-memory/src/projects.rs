use serde::{Deserialize, Serialize};
use skill_manager::projects::{
    usecase::{AddProject, DeleteProject, FindProjects, GetProject},
    Project, ProjectDescription, ProjectId, ProjectLabel,
};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Default, Deserialize, Serialize)]
pub struct ProjectDb(pub HashMap<ProjectId, Project>);

impl AddProject for ProjectDb {
    fn add(
        &mut self,
        label: ProjectLabel,
        description: ProjectDescription,
    ) -> skill_manager::Result<Project> {
        let id = ProjectId(Uuid::new_v4());
        let project = Project {
            id: id.clone(),
            label,
            description,
        };
        self.0.insert(id, project.clone());
        Ok(project)
    }
}

impl DeleteProject for ProjectDb {
    fn delete(&mut self, project_id: ProjectId) -> skill_manager::Result<()> {
        let _ = self.0.remove(&project_id);
        Ok(())
    }
}

impl GetProject for ProjectDb {
    fn get(&self, project_id: ProjectId) -> skill_manager::Result<Option<Project>> {
        Ok(self.0.get(&project_id).cloned())
    }
}

impl FindProjects for ProjectDb {
    fn find_projects(&self) -> skill_manager::Result<Vec<Project>> {
        Ok(self.0.values().cloned().collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use skill_manager::projects::{ProjectDescription, ProjectLabel};

    #[test]
    fn projects_api_test() -> skill_manager::Result<()> {
        let mut db = ProjectDb::default();
        let project = ProjectLabel("Example project".into());
        let added = db.add(project.clone(), ProjectDescription("".into()))?;
        assert_eq!(project, added.label);
        assert_eq!(db.get(added.id.clone())?, Some(added.clone()));
        db.delete(added.id.clone())?;
        assert_eq!(db.get(added.id)?, None);

        Ok(())
    }
}
