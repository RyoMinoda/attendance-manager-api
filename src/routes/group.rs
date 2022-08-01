
use actix_web::{HttpResponse, Responder, HttpRequest, http::header::ContentType};
use crate::{utils::request, repos::{group::GroupRepos, lib::Repos}, models::{lib::Responce, group::{GetGroupListResponce, GetGroupResponce}}};
use super::lib::ConnectionPool;

pub async fn list(pool: ConnectionPool) -> impl Responder {
    let repos = GroupRepos::new(pool);
    let groups = repos.get_all().to_list();
    let data = GetGroupListResponce::new(groups);
    let responce = Responce::<GetGroupListResponce>::new(data);
    HttpResponse::Ok()
        .content_type(ContentType::json())
        .body(responce)
}

pub async fn get(req: HttpRequest, pool: ConnectionPool) -> impl Responder {
    let id: String = request::get_param("id", req);
    println!("{}", id);
    let repos = GroupRepos::new(pool);
    let group = repos.get_by_id(id).first();
    let data = GetGroupResponce::new(group);
    let responce = Responce::<GetGroupResponce>::new(data);
    HttpResponse::Ok().body(responce)
}

pub async fn insert() -> impl Responder {
    HttpResponse::BadRequest().body("Can't Insert")
}
