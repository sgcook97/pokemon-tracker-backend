use crate::schema::cards;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = cards)]
#[diesel(belongs_to(Set, foreign_key = set_id))]
pub struct Card {
    pub card_id: String,
    pub set_id: Option<String>,
    pub name: String,
    pub rarity: Option<String>,
    pub number: Option<String>,
    pub img_small: Option<String>,
    pub img_hires: Option<String>,
    pub sort_key: Option<i32>,
}
