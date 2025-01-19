// @generated automatically by Diesel CLI.

diesel::table! {
    cards (card_id) {
        #[max_length = 255]
        card_id -> Varchar,
        #[max_length = 255]
        set_id -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        rarity -> Nullable<Varchar>,
        #[max_length = 10]
        number -> Nullable<Varchar>,
        #[max_length = 255]
        img_small -> Nullable<Varchar>,
        #[max_length = 255]
        img_hires -> Nullable<Varchar>,
        sort_key -> Integer
    }
}

diesel::table! {
    sets (set_id) {
        #[max_length = 255]
        set_id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        series -> Nullable<Varchar>,
        printed_total -> Nullable<Int4>,
        total -> Nullable<Int4>,
        release_date -> Nullable<Date>,
        updated_at -> Nullable<Timestamp>,
        #[max_length = 255]
        symbol_image_url -> Nullable<Varchar>,
        #[max_length = 255]
        logo_image_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    user_collections (user_id, set_id, card_id) {
        user_id -> Int4,
        #[max_length = 255]
        set_id -> Varchar,
        #[max_length = 255]
        card_id -> Varchar,
        quantity -> Nullable<Int4>,
    }
}

diesel::table! {
    users (user_id) {
        user_id -> Int4,
        email -> Varchar,
        password_hash -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(cards -> sets (set_id));
diesel::joinable!(user_collections -> cards (card_id));
diesel::joinable!(user_collections -> sets (set_id));
diesel::joinable!(user_collections -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(cards, sets, user_collections, users,);
