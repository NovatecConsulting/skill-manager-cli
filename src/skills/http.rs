use std::sync::Arc;

use serde::Deserialize;
use warp::{
    filters::{body, query::query},
    path::path,
    reply, Filter,
};

use crate::skills::{
    bl::{AddSkill, FindSkills, PageNumber, PageSize},
    SkillLabel,
};

#[derive(Deserialize)]
struct Request {
    label: SkillLabel,
}

pub fn add_skill(
    add_skill: impl AddSkill,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let add_skill = Arc::new(add_skill);
    warp::post()
        .and(path("api"))
        .and(path("skills"))
        .and(body::json())
        .map(
            move |skill_label: Request| match add_skill(skill_label.label) {
                Ok(skill) => reply::json(&skill),
                Err(e) => reply::json(&e),
            },
        )
}

#[derive(Deserialize)]
struct PageParams {
    #[serde(default)]
    page: PageNumber,
    #[serde(default)]
    size: PageSize,
}

pub fn find_skill(
    find_skill: impl FindSkills,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let find_skill = Arc::new(find_skill);
    warp::get()
        .and(path("api"))
        .and(path("skills"))
        .and(query())
        .map(move |page_params: PageParams| {
            let skill_page = find_skill(page_params.page, page_params.size);
            reply::json(&skill_page)
        })
}
