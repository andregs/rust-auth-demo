#![forbid(unsafe_code)]

pub mod config;
pub mod model;
mod controller;
mod service;
mod credential_repo;
mod token_repo;
mod tracer;

use credential_repo::*;
use model::*;
use service::*;
use token_repo::*;
use tracer::*;

// TODO keep only reusable parts on lib, move the app logic to bin crate
