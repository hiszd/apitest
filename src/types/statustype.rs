use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use serde::{Deserialize, Serialize};
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, AsExpression, FromSqlRow, PartialEq, Clone)]
#[diesel(sql_type = crate::schema::sql_types::Statustype)]
pub enum StatusType {
    Open,
    Closed,
    Working,
    Assigned,
    Unassigned,
}

impl ToSql<crate::schema::sql_types::Statustype, Pg> for StatusType {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            StatusType::Open => out.write_all(b"open")?,
            StatusType::Closed => out.write_all(b"closed")?,
            StatusType::Working => out.write_all(b"working")?,
            StatusType::Assigned => out.write_all(b"assigned")?,
            StatusType::Unassigned => out.write_all(b"unassigned")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<crate::schema::sql_types::Statustype, Pg> for StatusType {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"open" => Ok(StatusType::Open),
            b"closed" => Ok(StatusType::Closed),
            b"working" => Ok(StatusType::Working),
            b"assigned" => Ok(StatusType::Assigned),
            b"unassigned" => Ok(StatusType::Unassigned),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl From<String> for StatusType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "open" => StatusType::Open,
            "closed" => StatusType::Closed,
            "working" => StatusType::Working,
            "assigned" => StatusType::Assigned,
            "unassigned" => StatusType::Unassigned,
            _ => StatusType::Open,
        }
    }
}
