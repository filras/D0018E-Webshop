// @generated automatically by Diesel CLI.

diesel::table! {
    cart_items (user_id, item_id) {
        user_id -> Integer,
        item_id -> Integer,
        amount -> Integer,
    }
}

diesel::table! {
    comments (id) {
        id -> Integer,
        user_id -> Integer,
        review_id -> Integer,
        comment_id -> Nullable<Integer>,
        #[max_length = 255]
        comment -> Varchar,
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
    order_items (order_id, item_id) {
        order_id -> Integer,
        item_id -> Integer,
        amount -> Integer,
        total -> Integer,
    }
}

diesel::table! {
    orders (id) {
        id -> Integer,
        user_id -> Integer,
        #[max_length = 255]
        address -> Varchar,
        #[max_length = 255]
        co -> Nullable<Varchar>,
        #[max_length = 255]
        zipcode -> Varchar,
        #[max_length = 255]
        country -> Varchar,
        total -> Integer,
        #[max_length = 255]
        comment -> Nullable<Varchar>,
        payment_completed -> Bool,
    }
}

diesel::table! {
    reviews (id) {
        id -> Integer,
        user_id -> Integer,
        item_id -> Integer,
        rating -> Integer,
        #[max_length = 255]
        comment -> Nullable<Varchar>,
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

diesel::joinable!(cart_items -> items (item_id));
diesel::joinable!(cart_items -> users (user_id));
diesel::joinable!(comments -> reviews (review_id));
diesel::joinable!(comments -> users (user_id));
diesel::joinable!(order_items -> items (item_id));
diesel::joinable!(order_items -> orders (order_id));
diesel::joinable!(orders -> users (user_id));
diesel::joinable!(reviews -> items (item_id));
diesel::joinable!(reviews -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    cart_items,
    comments,
    items,
    order_items,
    orders,
    reviews,
    users,
);
