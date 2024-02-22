// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "statustype"))]
    pub struct Statustype;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "tickettype"))]
    pub struct Tickettype;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Statustype;
    use super::sql_types::Tickettype;

    tickets (id) {
        id -> Int4,
        subject -> Text,
        description -> Text,
        status -> Statustype,
        ticktype -> Tickettype,
        created_at -> Timestamp,
        updated_at -> Timestamp,
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
        email -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::joinable!(tickets_authors -> tickets (ticket_id));
diesel::joinable!(tickets_authors -> users (author_id));

diesel::allow_tables_to_appear_in_same_query!(tickets, tickets_authors, users);
