table! {
    category (id) {
        id -> Text,
        title -> Text,
        desc -> Text,
        link -> Text,
    }
}

table! {
    category_channel (category_id, channel_id) {
        category_id -> Text,
        channel_id -> Text,
    }
}

table! {
    channel (id) {
        id -> Text,
        title -> Text,
        desc -> Text,
        link -> Text,
        status -> Integer,
        item_count -> Integer,
    }
}

table! {
    feed (id) {
        id -> Text,
        channel_id -> Text,
        title -> Text,
        desc -> Text,
        link -> Text,
        pub_date -> Text,
        torrent_link -> Text,
        read_flag -> Bool,
    }
}

joinable!(category_channel -> category (category_id));
joinable!(category_channel -> channel (channel_id));
joinable!(feed -> channel (channel_id));

allow_tables_to_appear_in_same_query!(
    category,
    category_channel,
    channel,
    feed,
);
