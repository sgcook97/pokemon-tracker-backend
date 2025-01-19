use backend::models::{cards::Card, sets::Set};
use backend::schema::{cards, sets::dsl::*, sets::table as sets_table};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use regex::Regex;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::{env, error::Error};

#[derive(Deserialize)]
struct SetApiResponse {
    data: Vec<SetData>,
}

#[derive(Deserialize)]
struct SetData {
    id: String,
    name: String,
    series: Option<String>,
    #[serde(rename = "printedTotal")]
    printed_total: Option<i32>,
    total: Option<i32>,
    #[serde(rename = "releaseDate")]
    release_date: Option<String>,
    #[serde(rename = "updatedAt")]
    updated_at: Option<String>,
    images: SetImages,
}

#[derive(Deserialize)]
struct SetImages {
    symbol: Option<String>,
    logo: Option<String>,
}

#[derive(Deserialize)]
struct CardApiResponse {
    data: Vec<CardData>,
}

#[derive(Deserialize)]
struct CardData {
    id: String,
    name: String,
    rarity: Option<String>,
    number: Option<String>,
    images: CardImages,
    set: CardSet,
}

#[derive(Deserialize)]
struct CardImages {
    small: Option<String>,
    large: Option<String>,
}

#[derive(Deserialize)]
struct CardSet {
    id: Option<String>,
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    PgConnection::establish(&database_url).expect("Error connecting to databast")
}

fn generate_sort_key(card_num: &str) -> i32 {
    let reg_digits = Regex::new(r"^\d+$").unwrap();
    let reg_letters = Regex::new(r"^[A-Za-z]+$").unwrap();

    if reg_digits.is_match(card_num) {
        return card_num.parse::<i32>().unwrap_or(0);
    } else if reg_letters.is_match(card_num) {
        // arbitrary values chosen for sorting purposes
        if card_num == "ONE" {
            return 5000;
        } else if card_num == "TWO" {
            return 5001;
        } else if card_num == "THREE" {
            return 5002;
        } else if card_num == "FOUR" {
            return 5003;
        }
        let letter_val = card_num.chars().next().unwrap() as i32 - 'A' as i32 + 1;
        return letter_val + 1000;
    } else if card_num.contains('!') || card_num.contains('?') {
        // arbitrary values chosen for sorting purposes
        return 9999 + card_num.chars().next().unwrap() as i32;
    } else {
        return 0;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut connection = establish_connection();
    let client = Client::new();

    dotenv().ok();

    let api_url = env::var("POKEMON_TCG_URI").expect("POKEMON_TCG_URI must be set");
    let set_url = format!("{}/sets", &api_url);

    let set_response: SetApiResponse = client.get(&set_url).send()?.json()?;

    for set_data in set_response.data {
        let new_set = Set {
            set_id: set_data.id.clone(),
            name: set_data.name.clone(),
            series: set_data.series.clone(),
            printed_total: set_data.printed_total,
            total: set_data.total,
            release_date: set_data.release_date.as_ref().map(|date| {
                NaiveDate::parse_from_str(date, "%Y/%m/%d").expect("Invalid date format")
            }),
            updated_at: set_data.updated_at.as_ref().map(|date| {
                NaiveDateTime::parse_from_str(date, "%Y/%m/%d %H:%M:%S")
                    .expect("Invalid datetime format")
            }),
            symbol_image_url: set_data.images.symbol.clone(),
            logo_image_url: set_data.images.logo.clone(),
        };

        diesel::insert_into(sets_table)
            .values(&new_set)
            .on_conflict_do_nothing()
            .execute(&mut connection)?;
    }

    let all_set_ids: Vec<String> = sets.select(set_id).load::<String>(&mut connection)?;

    for set in all_set_ids.iter() {
        let cards_url = format!("{}/cards?q=set.id:{:?}", api_url, set);
        let card_response: CardApiResponse = client.get(&cards_url).send()?.json()?;

        for card_data in card_response.data {
            let card_num = card_data.number.as_deref().unwrap_or("");
            let sort_num: i32 = generate_sort_key(card_num);

            let new_card = Card {
                card_id: card_data.id.clone(),
                set_id: card_data.set.id.clone(),
                name: card_data.name.clone(),
                rarity: card_data.rarity.clone(),
                number: card_data.number.clone(),
                img_small: card_data.images.small.clone(),
                img_hires: card_data.images.large.clone(),
                sort_key: sort_num,
            };

            diesel::insert_into(cards::table)
                .values(&new_card)
                .on_conflict_do_nothing() // Avoid inserting duplicates
                .execute(&mut connection)?;
        }
    }

    Ok(())
}
