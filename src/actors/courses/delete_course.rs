use actix::Handler;
use diesel::{QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::{DbActor, messages::courses::delete_course::DeleteCourse, schema::courses::dsl::*};

impl Handler<DeleteCourse> for DbActor {
    type Result = QueryResult<usize>;
    fn handle(&mut self, msg: DeleteCourse, _ctx: &mut Self::Context) -> Self::Result {
        
        let mut conn = self.0.get().expect("DeleteCourse unable to establish connection");
        diesel::delete(courses.filter(id.eq(msg.course_id))).execute(&mut conn)
    }
}

