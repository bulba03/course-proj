use actix::Handler;
use diesel::{QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods};
use crate::{DbActor, messages::create_user::AuthUser, db_model::user::User, schema::users::dsl::*};

impl Handler<AuthUser> for DbActor {
    type Result = QueryResult<User>;
    fn handle(&mut self, msg: AuthUser, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("AuthUser unable to establish connection");
        users.filter(email.eq(msg.email)).get_result::<User>(&mut conn)
    }
}