use actix::Handler;
use diesel::{QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods, update};
use crate::{DbActor, messages::user_services::change_pass::ChangePass, schema::users::dsl::*, db_model::user::User};

impl Handler<ChangePass> for DbActor {
    type Result = QueryResult<usize>;
    fn handle(&mut self, msg: ChangePass, _ctx: &mut Self::Context) -> Self::Result {
        
        let mut conn = self.0.get().expect("ChangePass unable to establish connection");
        let users_query: QueryResult<User> = users.filter(id.eq(msg.user_id)).get_result::<User>(&mut conn);
        match users_query {
            Err(err) => return Err(err),
            Ok(user) => {
                if user.password.eq(&msg.prev_pass) {
                    return Err(diesel::result::Error::NotFound)
                }
            } 
        };
        update(users.filter(id.eq(msg.user_id))).set(password.eq(msg.new_pass)).execute(&mut conn)
    }
}

