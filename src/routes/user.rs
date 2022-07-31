use actix_web::{Responder, HttpResponse, get};


#[get("/users")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("H")
}