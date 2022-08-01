use r2d2::PooledConnection;
use diesel::{r2d2::ConnectionManager, PgConnection, Queryable};
use crate::{routes::lib::ConnectionPool};

pub type Pooled = PooledConnection<ConnectionManager<PgConnection>>;

pub trait Repos<T> {
    fn new(pool: ConnectionPool) -> Self;

    fn vec_update(self, vec: Vec<T>) -> Self;

    fn item_update(self, item: T) -> Self;

    fn get_connection(pool: ConnectionPool) -> Pooled {
        pool.get().expect("couldn't get db connection from pool")
    }

    fn get_all(self) -> Self;

    fn get_by_id(self, id: String) -> Self;

    fn insert_vec(self, vec: Vec<T>) -> Self;

    fn remove_all(self) -> Self;

    fn to_list(self) -> Vec<T>;

    fn first(self) -> T;
}