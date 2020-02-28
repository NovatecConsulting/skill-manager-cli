use warp::filters::body;
use warp::reply::json;
use warp::Filter;

use crate::skills::bl::IAddSkill;

struct Request<'a> {
    label: &'a str,
}

pub fn add_skill<F: IAddSkill>() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection>
{
    warp::post()
        .and(body::json())
        .map(|skill_label| match F::add_skill(skill_label) {
            Ok(skill) => json(&skill),
            Err(e) => json(&e),
        })
}
