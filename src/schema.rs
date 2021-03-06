table! {
    data (id) {
        id -> Int4,
        value -> Varchar,
        input_id -> Nullable<Int4>,
        script_id -> Nullable<Int4>,
    }
}

table! {
    data_tag (data_id, tag_id) {
        data_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    owner (id) {
        id -> Int4,
        external_id -> Varchar,
    }
}

table! {
    owner_tag (owner_id, tag_id) {
        owner_id -> Int4,
        tag_id -> Int4,
    }
}

table! {
    script (id) {
        id -> Int4,
        source -> Varchar,
        owner_id -> Nullable<Int4>,
    }
}

table! {
    script_tag (script_id, tag_id) {
        script_id -> Int4,
        tag_id -> Int4,
        is_output -> Bool,
    }
}

table! {
    tag (id) {
        id -> Int4,
        value -> Varchar,
        is_public -> Bool,
        owner_id -> Nullable<Int4>,
    }
}

joinable!(data -> script (script_id));
joinable!(data_tag -> data (data_id));
joinable!(data_tag -> tag (tag_id));
joinable!(owner_tag -> owner (owner_id));
joinable!(owner_tag -> tag (tag_id));
joinable!(script -> owner (owner_id));
joinable!(script_tag -> script (script_id));
joinable!(script_tag -> tag (tag_id));
joinable!(tag -> owner (owner_id));

allow_tables_to_appear_in_same_query!(
    data,
    data_tag,
    owner,
    owner_tag,
    script,
    script_tag,
    tag,
);
