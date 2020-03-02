use crate::skills::{Skill, SkillId, SkillLabel};

pub trait AddSkill = Fn(SkillLabel) -> Result<Skill, String>;

#[derive(Clone, Copy)]
pub struct PageNumber(usize);
#[derive(Clone, Copy)]
pub struct PageSize(usize);

pub trait FindSkills = Fn(Option<PageNumber>, Option<PageSize>) -> Result<Vec<Skill>, String>;

pub trait GetSkillById = Fn(SkillId) -> Result<Option<Skill>, String>;

pub trait DeleteSkillById = Fn(SkillId) -> Result<(), String>;

#[cfg(test)]
pub mod in_memory {
    use std::{cell::RefCell, collections::HashMap, rc::Rc};

    use uuid::Uuid;

    use super::*;

    pub fn api() -> SkillsApi {
        let db = Rc::new(RefCell::new(HashMap::new()));
        let get = get(db.clone());
        let find = find(db.clone());
        let add = add(db.clone());
        let delete = delete(db);
        SkillsApi {
            get,
            find,
            add,
            delete,
        }
    }

    #[test]
    fn skill_api_test() -> Result<(), String> {
        let api = api();
        let skill = SkillLabel("Example".into());
        assert_eq!((api.find)(None, None)?, vec![]);
        let added_skill = (api.add)(skill.clone())?;
        assert_eq!(added_skill.label, skill);
        Ok(())
    }

    pub struct SkillsApi {
        pub get: Box<dyn GetSkillById>,
        pub find: Box<dyn FindSkills>,
        pub add: Box<dyn AddSkill>,
        pub delete: Box<dyn DeleteSkillById>,
    }

    type Db = Rc<RefCell<HashMap<SkillId, Skill>>>;

    fn get(db: Db) -> Box<dyn GetSkillById> {
        Box::new(move |skill_id| Ok(db.borrow().get(&skill_id).cloned()))
    }

    fn find(db: Db) -> Box<dyn FindSkills> {
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

    fn add(db: Db) -> Box<dyn AddSkill> {
        Box::new(move |label| {
            let id = SkillId(Uuid::new_v4());
            let skill = Skill {
                //id: id.clone(),
                label,
            };
            let _ = db.clone().borrow_mut().insert(id, skill.clone());
            Ok(skill)
        })
    }

    fn delete(db: Db) -> Box<dyn DeleteSkillById> {
        Box::new(move |skill_id| {
            let _ = db.clone().borrow_mut().remove(&skill_id);
            Ok(())
        })
    }
}
