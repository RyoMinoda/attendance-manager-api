use actix_web::{web};
use crate::configs::postgres::Pool;
use super::{group, seed};

pub type ConnectionPool = web::Data<Pool>;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/groups/{id}", web::get().to(group::get))
        .route("/groups", web::get().to(group::list))
        .route("/groups/{id}/insert", web::post().to(group::insert))


        .route("/seed", web::get().to(seed::seed));
}
