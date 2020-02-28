#![feature(trait_alias)]
use uuid::Uuid;

mod skills;

use skills::{bl, http, persist};

#[tokio::main]
async fn main() {
    let add_skill = http::add_skill(bl::add_skill(
        Uuid::new_v4,
        persist::insert_skill_into_repository(),
    ));
    warp::serve(add_skill).run(([0, 0, 0, 0], 8080)).await;
}
