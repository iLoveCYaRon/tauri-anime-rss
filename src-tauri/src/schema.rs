// @generated automatically by Diesel CLI.

diesel::table! {
    category (id) {
        id -> Text,
        title -> Text,
        desc -> Text,
        link -> Text,
    }
}

diesel::table! {
    category_channel (category_id, channel_id) {
        category_id -> Text,
        channel_id -> Text,
    }
}

diesel::table! {
    channel (id) {
        id -> Text,
        title -> Text,
        desc -> Text,
        link -> Text,
        status -> Integer,
        item_count -> Integer,
    }
}

diesel::table! {
    channel_item (id) {
        id -> Text,
        channel_id -> Text,
        title -> Text,
        desc -> Text,
        link -> Text,
        pub_date -> Text,
        torrent_link -> Text,
    }
}

diesel::joinable!(category_channel -> category (category_id));
diesel::joinable!(category_channel -> channel (channel_id));
diesel::joinable!(channel_item -> channel (channel_id));

diesel::allow_tables_to_appear_in_same_query!(
    category,
    category_channel,
    channel,
    channel_item,
);
