// @generated automatically by Diesel CLI.

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
