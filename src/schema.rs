table! {
    input (id) {
        id -> Int4,
        data -> Varchar,
    }
}

table! {
    input_tag (input_id, tag_id) {
        input_id -> Int4,
        tag_id -> Varchar,
    }
}

table! {
    output (script_id, input_id) {
        data -> Varchar,
        script_id -> Int4,
        input_id -> Int4,
    }
}

table! {
    script (id) {
        id -> Int4,
        lang -> Varchar,
        source -> Varchar,
    }
}

table! {
    script_tag (script_id, tag_id) {
        script_id -> Int4,
        tag_id -> Varchar,
    }
}

table! {
    tag (id) {
        id -> Varchar,
    }
}

joinable!(input_tag -> input (input_id));
joinable!(input_tag -> tag (tag_id));
joinable!(output -> input (input_id));
joinable!(output -> script (script_id));
joinable!(script_tag -> script (script_id));
joinable!(script_tag -> tag (tag_id));

allow_tables_to_appear_in_same_query!(
    input,
    input_tag,
    output,
    script,
    script_tag,
    tag,
);
