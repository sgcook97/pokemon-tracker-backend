use crate::schema::users;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: i32,
    pub email: String,
    pub password_hash: String,
    pub created_at: Option<NaiveDateTime>,
}
