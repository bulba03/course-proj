use actix::Message;

use diesel::QueryResult;
use crate::db_model::course::Course;
#[derive(Message)]
#[rtype(result = "QueryResult<Course>")]
pub struct EditCourse{
    pub course_id: i32,
    pub name: String,
    pub desctiption: String,
    pub teacher_id: i32,
}