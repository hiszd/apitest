// @generated automatically by Diesel CLI.

diesel::table! {
    tickets (id) {
        id -> Int4,
        number -> Int4,
        subject -> Text,
        user_id -> Int4,
    }
}

diesel::table! {
    tickets_authors (ticket_id, author_id) {
        ticket_id -> Int4,
        author_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(tickets -> users (user_id));
diesel::joinable!(tickets_authors -> tickets (ticket_id));
diesel::joinable!(tickets_authors -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(tickets, tickets_authors, users,);
