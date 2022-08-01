table! {
    groups (group_id) {
        group_id -> Varchar,
        name -> Text,
        detail -> Text,
        is_active -> Bool,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}
