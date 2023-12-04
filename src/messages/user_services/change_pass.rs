use actix::Message;

use diesel::QueryResult;
#[derive(Message)]
#[rtype(result = "QueryResult<usize>")]
pub struct ChangePass{
    pub prev_pass: String,
    pub new_pass: String,
    pub user_id: i32
}