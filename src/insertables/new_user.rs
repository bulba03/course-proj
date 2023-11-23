use diesel::deserialize::{self, FromSql, FromSqlRow};
use diesel::expression::AsExpression;
use diesel::pg::{Pg, PgValue}; 
use diesel::serialize::{self, IsNull, Output, ToSql};
use std::io::Write;
use diesel::Insertable;
use serde::Serialize;

use crate::schema::users; 

#[derive(Debug, AsExpression, FromSqlRow, Serialize, Clone)]
#[diesel(sql_type = crate::schema::sql_types::RoleEnum)]
pub enum RoleEnum {
    Admin,
    User,
    Teacher
}
#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub role: RoleEnum,
}
impl ToSql<crate::schema::sql_types::RoleEnum, Pg> for RoleEnum {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        match *self {
            RoleEnum::Admin => out.write_all(b"admin")?,
            RoleEnum::User => out.write_all(b"user")?,
            RoleEnum::Teacher => out.write_all(b"teacher")?,
        }
        Ok(IsNull::No)
    }
}
impl FromSql<crate::schema::sql_types::RoleEnum, Pg> for RoleEnum {
    fn from_sql(bytes: PgValue) -> deserialize::Result<Self> {
        match bytes.as_bytes() {
            b"admin" => Ok(RoleEnum::Admin),
            b"user" => Ok(RoleEnum::User),
            b"teacher" => Ok(RoleEnum::Teacher),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}
