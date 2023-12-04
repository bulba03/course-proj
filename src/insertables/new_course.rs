use diesel::Insertable;
use serde::Serialize;

use crate::schema::courses;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name = courses)]

pub struct NewCourse {
    pub name: String,
    pub description: String,
    pub teacher_id: i32
}