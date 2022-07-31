use std::env;

use diesel::{r2d2::ConnectionManager, PgConnection};
use dotenv::dotenv;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_pool() -> Pool {
    println!("get_pool_start");
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("couldn't build connection pool")
}
