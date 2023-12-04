use actix::Message;

use diesel::QueryResult;
use crate::db_model::lesson::Lesson;
#[derive(Message)]
#[rtype(result = "QueryResult<Lesson>")]
pub struct CreateLesson {
    pub name: String,
    pub description: String,
    pub resource_link: String, 
    pub course_id: i32
}
