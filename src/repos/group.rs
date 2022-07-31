use diesel::RunQueryDsl;

use super::lib::{Repos, Pooled};
use crate::{routes::lib::ConnectionPool, models::group::Group, configs, schema::groups};

pub struct GroupRepos {
    conn: Pooled,
    list: Vec<Group>,
    mock: bool
}

impl Repos<Group> for GroupRepos {
    fn new(pool: ConnectionPool) -> Self {
        GroupRepos { 
            conn: Self::get_connection(pool), 
            list: Vec::new(),
            mock: configs::repos::get_mock(),
        }
    }

    fn vec_update(self, vec: Vec<Group>) -> Self {
        GroupRepos {
            conn: self.conn,
            list: vec,
            mock: self.mock 
        }
    }

    fn get_connection(pool: ConnectionPool) -> Pooled {
        pool.get().expect("couldn't get db connection from pool")
    }

    fn to_list(self) -> Vec<Group> {
        self.list
    }

    fn get_all(self) -> Self {
        use crate::schema::groups::dsl::groups;
        let vec = groups.load::<Group>(&self.conn).expect("");
        self.vec_update(vec)
    }
}