use skill_manager::skills::{
    usecase::{AddSkill, DeleteSkillById, FindSkills, GetSkillById, PageNumber, PageSize},
    Skill, SkillId,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use uuid::Uuid;

pub fn skills_api() -> SkillsApi {
    let db = Rc::new(RefCell::new(HashMap::new()));
    skills_api_using(db)
}

pub fn skills_api_using(db: SkillDb) -> SkillsApi {
    let get = get(db.clone());
    let find = find(db.clone());
    let add = add(db.clone());
    let delete = delete(db.clone());
    SkillsApi {
        db,
        get,
        find,
        add,
        delete,
    }
}

pub struct SkillsApi {
    pub db: SkillDb,
    pub get: Box<dyn GetSkillById>,
    pub find: Box<dyn FindSkills>,
    pub add: Box<dyn AddSkill>,
    pub delete: Box<dyn DeleteSkillById>,
}

pub type SkillDb = Rc<RefCell<SkillStore>>;
pub type SkillStore = HashMap<SkillId, Skill>;

fn get(db: SkillDb) -> Box<dyn GetSkillById> {
    Box::new(move |skill_id| Ok(db.borrow().get(&skill_id).cloned()))
}

fn find(db: SkillDb) -> Box<dyn FindSkills> {
    Box::new(
        move |page_number: Option<PageNumber>, page_size: Option<PageSize>| {
            let page_number = match page_number {
                Some(PageNumber(n)) => n,
                None => 0,
            };
            let page_size = match page_size {
                Some(PageSize(n)) => n,
                None => 10,
            };
            Ok(db
                .borrow()
                .values()
                .skip(page_number * page_size)
                .take(page_size)
                .cloned()
                .collect())
        },
    )
}

fn add(db: SkillDb) -> Box<dyn AddSkill> {
    Box::new(move |label| {
        let id = SkillId(Uuid::new_v4());
        let skill = Skill {
            id: id.clone(),
            label,
        };
        let _ = db.clone().borrow_mut().insert(id, skill.clone());
        Ok(skill)
    })
}

fn delete(db: SkillDb) -> Box<dyn DeleteSkillById> {
    Box::new(move |skill_id| {
        let _ = db.clone().borrow_mut().remove(&skill_id);
        Ok(())
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn skill_api_test() -> crate::Result<()> {
        use crate::skills::SkillLabel;

        let api = skills_api();
        let skill = SkillLabel("Example".into());
        assert_eq!((api.find)(None, None)?, vec![]);
        let added_skill = (api.add)(skill.clone())?;
        assert_eq!(added_skill.label, skill);
        Ok(())
    }
}
