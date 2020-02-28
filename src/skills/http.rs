use std::sync::Arc;

use serde::Deserialize;
use warp::{filters::body, reply::json, Filter};

use crate::skills::{bl::AddSkill, SkillLabel};

#[derive(Deserialize)]
struct Request {
    label: SkillLabel,
}

pub fn add_skill(
    add_skill: impl AddSkill,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let add_skill = Arc::new(add_skill);
    warp::post()
        .and(body::json())
        .map(
            move |skill_label: Request| match add_skill(skill_label.label) {
                Ok(skill) => json(&skill),
                Err(e) => json(&e),
            },
        )
}
