use diesel::Insertable;
use serde::Serialize;

use crate::schema::lessons;

#[derive(Insertable, Serialize, Clone)]
#[diesel(table_name = lessons)]

pub struct NewLesson {
    pub name: String,
    pub description: String,
    pub course_id: i32,
    pub resource_link: String
}