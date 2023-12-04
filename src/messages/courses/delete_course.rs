use actix::Message;

use diesel::QueryResult;
#[derive(Message)]
#[rtype(result = "QueryResult<usize>")]
pub struct DeleteCourse{
    pub course_id: i32
}