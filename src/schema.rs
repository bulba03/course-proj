// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role_enum"))]
    pub struct RoleEnum;
}

diesel::table! {
    courses (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        teacher_id -> Nullable<Int4>,
    }
}

diesel::table! {
    lessons (id) {
        id -> Int4,
        course_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        resource_link -> Nullable<Text>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::RoleEnum;

    users (id) {
        id -> Int4,
        #[max_length = 255]
        first_name -> Varchar,
        #[max_length = 255]
        last_name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        role -> RoleEnum,
    }
}

diesel::joinable!(courses -> users (teacher_id));
diesel::joinable!(lessons -> courses (course_id));

diesel::allow_tables_to_appear_in_same_query!(
    courses,
    lessons,
    users,
);
