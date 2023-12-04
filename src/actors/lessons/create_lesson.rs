use actix::Handler;
use diesel::{QueryResult, RunQueryDsl};
use crate::{DbActor, messages::{courses::create_course::CreateCourse, lessons::create_lesson::CreateLesson}, insertables, db_model::{course::Course, lesson::Lesson}, schema::{courses::dsl::*, lessons}};

impl Handler<CreateLesson> for DbActor {
    type Result = QueryResult<Lesson>;
    fn handle(&mut self, msg: CreateLesson, _ctx: &mut Self::Context) -> Self::Result {
        
        let mut conn = self.0.get().expect("CreateCourse unable to establish connection");
        let new_lesson = insertables::new_lesson::NewLesson {
            name: msg.name,
            description: msg.description,
            resource_link: msg.resource_link,
            course_id: msg.course_id,
        };
        diesel::insert_into(lessons::table).values(new_lesson).get_result::<Lesson>(&mut conn)
    }
}

