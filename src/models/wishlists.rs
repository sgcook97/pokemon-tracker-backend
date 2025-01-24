use crate::schema::wishlists;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = wishlists)]
#[diesel(belongs_to(User, foreign_key = user_id))]
pub struct Wishlist {
    pub user_id: i32,
    pub wishlist: serde_json::Value,
}
