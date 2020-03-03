#![feature(trait_alias)]

use thiserror::Error;

#[macro_use]
mod wrapper;
pub mod employees;
pub mod in_memory;
pub mod projects;
pub mod skills;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Error: {0}")]
    Other(String),
}
type Result<T> = std::result::Result<T, Error>;
