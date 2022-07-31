
use actix_web::{HttpResponse, Responder, HttpRequest, web};
use crate::{utils::request, repos::{group::GroupRepos, lib::Repos}, models::{lib::Responce, group::{Group, GetGroupListResponce}}};
use super::lib::ConnectionPool;

async fn list(pool: ConnectionPool) -> impl Responder {
    let repos = GroupRepos::new(pool);
    let groups = repos.get_all().to_list();
    let data = GetGroupListResponce::new(groups);
    let responce = Responce::<GetGroupListResponce>::new(data);
    HttpResponse::Ok().body(responce)
}

async fn get(req: HttpRequest) -> impl Responder {
    let target: String = request::get_param("id", req);
    HttpResponse::Ok().body(target)
}

async fn insert() -> impl Responder {
    HttpResponse::BadRequest().body("Can't Insert")
}


pub fn config_groups(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/groups/{id}", web::get().to(get))
        .route("/groups", web::get().to(list))
        .route("/groups/{id}/insert", web::post().to(insert));
}
