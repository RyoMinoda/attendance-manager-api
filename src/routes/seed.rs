use actix_web::{Responder, HttpResponse};

use crate::{repos::{group::GroupRepos, lib::Repos}, models::{group::{Group}, lib::Seed}};

use super::lib::ConnectionPool;

pub async fn seed(pool: ConnectionPool) -> impl Responder {
    let repos = GroupRepos::new(pool);
    let groups = Group::get_seed();
    repos
        .remove_all()
        .insert_vec(groups);
    HttpResponse::Ok().body("Seed OK")
}


