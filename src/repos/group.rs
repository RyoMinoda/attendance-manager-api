use diesel::{RunQueryDsl, QueryDsl};

use super::lib::{Repos, Pooled};
use crate::{routes::lib::ConnectionPool, models::group::{Group}, configs};
use diesel::expression_methods::*;

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

    fn item_update(self, item: Group) -> Self {
        GroupRepos { 
            conn: self.conn, 
            list: vec![item], 
            mock: self.mock 
        }
    }

    fn get_connection(pool: ConnectionPool) -> Pooled {
        pool.get().expect("couldn't get db connection from pool")
    }

    fn to_list(self) -> Vec<Group> {
        self.list
    }

    fn first(self) -> Group {
        self.list.into_iter().nth(0).expect("couldn't get first")
    }

    fn get_all(self) -> Self {
        use crate::schema::groups::dsl::groups;
        let vec = groups.load::<Group>(&self.conn).expect("");
        self.vec_update(vec)
    }

    fn get_by_id(self, id: String) -> Self {
        use crate::schema::groups::dsl::{groups, group_id};
        let target = groups
            .filter(group_id.eq(id))
            .first(&self.conn)
            .expect("couldn't get group by id");
        self.item_update(target)
    }

    fn insert_vec(self, vec: Vec<Group>) -> Self {
        use crate::schema::groups::dsl::groups;
        diesel::insert_into(groups)
            .values(vec)
            .execute(&self.conn)
            .expect("couldn't insert groups");
        self
    }

    fn remove_all(self) -> Self {
        use crate::schema::groups::dsl::groups;
        diesel::delete(groups)
            .execute(&self.conn)
            .expect("couldn't delete groups");
        self
    }
}