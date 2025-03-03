// @generated automatically by Diesel CLI.

diesel::table! {
    cart_items (user_id, item_id) {
        user_id -> Integer,
        item_id -> Integer,
        amount -> Integer,
    }
}

diesel::table! {
    items (id) {
        id -> Integer,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        price -> Integer,
        in_stock -> Integer,
        average_rating -> Nullable<Float>,
        discounted_price -> Nullable<Integer>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        #[max_length = 255]
        firstname -> Varchar,
        #[max_length = 255]
        surname -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        role -> Varchar,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        #[max_length = 255]
        zipcode -> Nullable<Varchar>,
        #[max_length = 255]
        co -> Nullable<Varchar>,
        #[max_length = 255]
        country -> Nullable<Varchar>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(cart_items, items, users,);

diesel::joinable!(cart_items -> items (item_id));
