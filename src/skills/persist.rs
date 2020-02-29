use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use crate::skills::{
    bl::{FindSkills, PageNumber, PageSize},
    Skill,
};

pub trait PersistSkill = Fn(Skill) -> Result<(), String> + Sync + Send + 'static;

pub fn insert_skill_into_hashmap(store: Arc<Mutex<HashMap<Uuid, Skill>>>) -> impl PersistSkill {
    move |skill: Skill| {
        store.lock().unwrap().insert(skill.id.clone(), skill);
        Ok(())
    }
}

pub fn find_skills_in_hashmap(store: Arc<Mutex<HashMap<Uuid, Skill>>>) -> impl FindSkills {
    move |page_number: PageNumber, page_size: PageSize| {
        Ok(store
            .lock()
            .unwrap()
            .values()
            .skip(page_number.0 * page_size.0)
            .take(page_size.0)
            .cloned()
            .collect())
    }
}
