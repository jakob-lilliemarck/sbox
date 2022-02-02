table! {
    input_tags (input_id, tag_id) {
        input_id -> Int4,
        tag_id -> Varchar,
    }
}

table! {
    inputs (id) {
        id -> Int4,
        data -> Varchar,
        tags -> Varchar,
        output_id -> Nullable<Int4>,
    }
}

table! {
    outputs (id) {
        id -> Int4,
        data -> Varchar,
    }
}

table! {
    source (id) {
        id -> Int4,
        lang -> Varchar,
        src -> Varchar,
        output_id -> Nullable<Int4>,
    }
}

table! {
    source_tags (source_id, tag_id) {
        source_id -> Int4,
        tag_id -> Varchar,
    }
}

table! {
    tags (id) {
        id -> Varchar,
    }
}

joinable!(input_tags -> inputs (input_id));
joinable!(input_tags -> tags (tag_id));
joinable!(inputs -> outputs (output_id));
joinable!(source -> outputs (output_id));
joinable!(source_tags -> source (source_id));
joinable!(source_tags -> tags (tag_id));

allow_tables_to_appear_in_same_query!(
    input_tags,
    inputs,
    outputs,
    source,
    source_tags,
    tags,
);
