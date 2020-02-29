#![feature(trait_alias)]
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use warp::Filter;

mod skills;

use skills::{bl, http, persist};

#[tokio::main]
async fn main() {
    let store = Arc::new(Mutex::new(HashMap::new()));
    let add_skill = http::add_skill(bl::add_skill(
        Uuid::new_v4,
        persist::insert_skill_into_hashmap(store.clone()),
    ));
    let find_skills = http::find_skill(persist::find_skills_in_hashmap(store.clone()));
    warp::serve(add_skill.or(find_skills))
        .run(([0, 0, 0, 0], 8080))
        .await;
}
