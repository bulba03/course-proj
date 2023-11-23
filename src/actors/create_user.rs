use std::string;

use actix::Handler;
use diesel::{QueryResult, RunQueryDsl};
use crate::{DbActor, messages::create_user::CreateUser, db_model::user::User, schema::users::dsl::*, insertables::{self, new_user::RoleEnum}};

impl Handler<CreateUser> for DbActor {
    type Result = QueryResult<User>;
    fn handle(&mut self, msg: CreateUser, ctx: &mut Self::Context) -> Self::Result {
        let mut conn = self.0.get().expect("NewUser unable to establish connection");
        let new_user = insertables::new_user::NewUser{
            first_name: msg.first_name,
            last_name: msg.last_name,
            email: msg.email,
            password: msg.password,
            role: RoleEnum::User
        };
        diesel::insert_into(users).values(new_user).get_result::<User>(&mut conn)
    }
}