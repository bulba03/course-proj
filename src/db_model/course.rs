use diesel::Queryable;
use serde::Serialize;
#[derive(Queryable, Debug, Serialize)]
pub struct Course {
    id: i32,
    name: String,
    description: Option<String>,
    teacher: Option<i32>,
}