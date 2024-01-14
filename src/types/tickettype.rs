use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, AsExpression, FromSqlRow, PartialEq, Clone)]
#[diesel(sql_type = crate::schema::sql_types::Tickettype)]
pub enum TicketType {
    Hardware,
    Software,
    Email,
    Employee,
}

impl ToSql<crate::schema::sql_types::Tickettype, Pg> for TicketType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            TicketType::Hardware => out.write_all(b"hardware")?,
            TicketType::Software => out.write_all(b"software")?,
            TicketType::Email => out.write_all(b"email")?,
            TicketType::Employee => out.write_all(b"employee")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Tickettype, Pg> for TicketType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"hardware" => Ok(TicketType::Hardware),
            b"software" => Ok(TicketType::Software),
            b"email" => Ok(TicketType::Email),
            b"employee" => Ok(TicketType::Employee),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl From<String> for TicketType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "hardware" => TicketType::Hardware,
            "software" => TicketType::Software,
            "email" => TicketType::Email,
            "employee" => TicketType::Employee,
            _ => TicketType::Hardware,
        }
    }
}
