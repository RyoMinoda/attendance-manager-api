use std::time::SystemTime;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Deserialize, Serialize, PostgresMapper, Queryable )]
#[pg_mapper(table = "groups")]
pub struct Group {
    pub group_id: String,
    pub name: String,
    pub detail: String,
    pub created_at: SystemTime
}

impl Group {
    pub fn new(name: &str) -> Self {
        Group {
            group_id: Uuid::new_v4().to_string(),
            name: name.to_string(),
            detail: "".to_string(),
            created_at: SystemTime::now()
        }
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