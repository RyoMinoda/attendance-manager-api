use std::time::SystemTime;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use crate::schema::groups;
use super::lib::Seed;

#[derive(Deserialize, Serialize, PostgresMapper, Queryable, Insertable )]
#[pg_mapper(table = "groups")]
pub struct Group {
    pub group_id: String,
    pub name: String,
    pub detail: String,
    pub is_active: bool,
    pub updated_at: SystemTime,
    pub created_at: SystemTime
}

impl Group {
    pub fn new(name: &str) -> Self {
        Group {
            group_id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            detail: "".to_string(),
            is_active: true,
            updated_at: SystemTime::now(),
            created_at: SystemTime::now()
        }
    }
}


impl Seed<Group> for Group {
    fn get_seed() -> Vec<Group> {
        Vec::from([
            Group::new("group1"),
            Group::new("group2"),
            Group::new("group3"),
        ])
    }
}

#[derive(Deserialize, Serialize)]
pub struct GetGroupListResponce {
    pub groups: Vec<Group>
}

impl GetGroupListResponce {
    pub fn new(groups: Vec<Group>) -> Self {
        GetGroupListResponce { groups: groups }
    }
}

#[derive(Deserialize, Serialize)]
pub struct GetGroupResponce {
    pub group: Group
}

impl GetGroupResponce {
    pub fn new(group: Group) -> Self {
        GetGroupResponce { group: group }
    }
}