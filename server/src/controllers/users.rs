use super::super::schema::*;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct UserChange {
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
