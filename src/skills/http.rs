use warp::filters::body;
use warp::reply::json;
use warp::Filter;

use crate::skills::bl::AddSkill;

struct Request<'a> {
    label: &'a str,
}

pub fn add_skill<F: AddSkill>() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection>
{
    warp::post()
        .and(body::json())
        .map(|skill_label| match F::add_skill(skill_label) {
            Ok(skill) => json(&skill),
            Err(e) => json(&e),
        })
}
