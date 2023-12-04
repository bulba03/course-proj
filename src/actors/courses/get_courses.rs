use actix::Handler;
use diesel::{QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::{DbActor, messages::courses::get_courses::GetCourse, db_model::course::Course, schema::courses::dsl::*};

impl Handler<GetCourse> for DbActor {
    type Result = QueryResult<Vec<Course>>;
    fn handle(&mut self, msg: GetCourse, _ctx: &mut Self::Context) -> Self::Result {
        
        let mut conn = self.0.get().expect("GetCourse unable to establish connection");
        if msg.is_all {
            return courses.load(&mut conn);
        }
        else {
            return courses.filter(id.eq(msg.course_id)).get_results::<Course>(&mut conn);
        }
    }
}

