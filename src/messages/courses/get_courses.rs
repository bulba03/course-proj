use actix::Message;

use diesel::QueryResult;
use crate::db_model::course::Course;
#[derive(Message)]
#[rtype(result = "QueryResult<Vec<Course>>")]
pub struct GetCourse{
   pub course_id: i32,
   pub is_all: bool
}