use crate::schema::sets;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = sets)]
pub struct Set {
    pub set_id: String,
    pub name: String,
    pub series: Option<String>,
    pub printed_total: Option<i32>,
    pub total: Option<i32>,
    pub release_date: Option<NaiveDate>,
    pub updated_at: Option<NaiveDateTime>,
    pub symbol_image_url: Option<String>,
    pub logo_image_url: Option<String>,
}
