// @generated automatically by Diesel CLI.

diesel::table! {
    use diesel::sql_types::{ Int4, Text };
    use crate::TicketTypeMapping;
    tickets (id) {
        id -> Int4,
        index -> Int4,
        subject -> Text,
        description -> Text,
        ticktype -> TicketTypeMapping,
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

diesel::joinable!(tickets_authors -> tickets (ticket_id));
diesel::joinable!(tickets_authors -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(tickets, tickets_authors, users,);
