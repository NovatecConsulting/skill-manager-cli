use serde::{Deserialize, Serialize};
use skill_manager::skills::{
    usecase::{AddSkill, DeleteSkillById, FindSkills, GetSkillById},
    Skill, SkillId, SkillLabel,
};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Clone, Deserialize, Serialize, Default)]
pub struct SkillDb(pub HashMap<SkillId, Skill>);

impl GetSkillById for SkillDb {
    fn get(&self, skill_id: SkillId) -> skill_manager::Result<Option<Skill>> {
        Ok(self.0.get(&skill_id).cloned())
    }
}

impl FindSkills for SkillDb {
    fn find(&self) -> skill_manager::Result<Vec<Skill>> {
        Ok(self.0.values().cloned().collect())
    }
}

impl AddSkill for SkillDb {
    fn add(&mut self, label: SkillLabel) -> skill_manager::Result<Skill> {
        let id = SkillId(Uuid::new_v4());
        let skill = Skill {
            id: id.clone(),
            label,
        };
        let _ = self.0.insert(id, skill.clone());
        Ok(skill)
    }
}

impl DeleteSkillById for SkillDb {
    fn delete(&mut self, skill_id: SkillId) -> skill_manager::Result<()> {
        let _ = self.0.remove(&skill_id);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use skill_manager::skills::SkillLabel;

    #[test]
    fn skill_api_test() -> skill_manager::Result<()> {
        let mut db = SkillDb::default();
        let skill = SkillLabel("Example".into());

        assert_eq!(db.find()?, vec![]);

        let added_skill = db.add(skill.clone())?;

        assert_eq!(added_skill.label, skill);
        assert_eq!(db.get(added_skill.id.clone())?.unwrap(), added_skill);
        assert_eq!(db.find()?, vec![added_skill.clone()]);

        db.delete(added_skill.id.clone())?;

        assert_eq!(db.find()?, vec![]);
        assert_eq!(db.get(added_skill.id)?, None);

        Ok(())
    }
}
