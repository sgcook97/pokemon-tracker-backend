use crate::schema::custom_collections;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = custom_collections)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct CustomCollection {
    pub collection_id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub custom_collection: serde_json::Value,
}
