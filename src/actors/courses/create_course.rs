use actix::Handler;
use diesel::{QueryResult, RunQueryDsl};
use crate::{DbActor, messages::courses::create_course::CreateCourse, insertables, db_model::course::Course, schema::courses::dsl::*};

impl Handler<CreateCourse> for DbActor {
    type Result = QueryResult<Course>;
    fn handle(&mut self, msg: CreateCourse, _ctx: &mut Self::Context) -> Self::Result {
        
        let mut conn = self.0.get().expect("CreateCourse unable to establish connection");
        let new_course = insertables::new_course::NewCourse {
            name: msg.name,
            description: msg.desctiption,
            teacher_id: msg.teacher_id
        };
        diesel::insert_into(courses).values(new_course).get_result::<Course>(&mut conn)
    }
}

