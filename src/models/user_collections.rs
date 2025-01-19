use crate::schema::user_collections;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = user_collections)]
#[diesel(belongs_to(User, foreign_key = user_id))]
#[diesel(belongs_to(Set, foreign_key = set_id))]
#[diesel(belongs_to(Card, foreign_key = card_id))]
struct UserCollection {
    pub user_id: i32,
    pub card_id: String,
    pub set_id: String,
    pub quantity: Option<i32>,
}
