use crate::skills::Skill;

pub trait PersistSkill = Fn(&Skill) -> Result<(), String> + Sync + Send + 'static;

pub fn insert_skill_into_repository() -> Box<dyn PersistSkill> {
    Box::new(|skill| {
        println!("Persisting skill {:?}", skill);
        Ok(())
    })
}
