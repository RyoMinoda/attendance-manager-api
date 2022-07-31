use actix_web::{web};
use crate::configs::postgres::Pool;

pub type ConnectionPool = web::Data<Pool>;

