table! {
    attrs (attr_id) {
        attr_id -> Integer,
        id -> Integer,
        key -> Text,
        val -> Text,
    }
}

table! {
    logs (id) {
        id -> Integer,
        name -> Text,
        desc -> Text,
        time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    attrs,
    logs,
);
