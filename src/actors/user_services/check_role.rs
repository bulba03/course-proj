use actix::Handler;
use diesel::{QueryResult, RunQueryDsl, QueryDsl, ExpressionMethods, BoolExpressionMethods};
use crate::{DbActor, messages::user_services::check_role::CheckUserRole, db_model::user::User, schema::users::dsl::*};

impl Handler<CheckUserRole> for DbActor {
    type Result = QueryResult<User>;
    fn handle(&mut self, msg: CheckUserRole, _ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("AuthUser unable to establish connection");
        users.filter(id.eq(msg.user_id).and(role.eq(msg.needed_role))).get_result::<User>(&mut conn)
    }
}