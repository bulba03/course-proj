use actix::Handler;
use diesel::{QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::{DbActor, messages::courses::edit_course::EditCourse, db_model::course::Course, schema::courses::dsl::*};

impl Handler<EditCourse> for DbActor {
    type Result = QueryResult<Course>;
    fn handle(&mut self, msg: EditCourse, _ctx: &mut Self::Context) -> Self::Result {
        
        let mut conn = self.0.get().expect("EditCourse unable to establish connection");
        diesel::update(courses.filter(id.eq(msg.course_id)))
            .set((
                name.eq(msg.name.clone()),
                description.eq(msg.desctiption.clone()),
                teacher_id.eq(msg.teacher_id),
            ))
            .get_result::<Course>(&mut conn)
    }
}

