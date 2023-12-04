use diesel::Queryable;
use serde::Serialize;
#[derive(Queryable, Debug, Serialize)]
pub struct Lesson {
    id: i32,
    course_id: i32,
    name: String,
    description: Option<String>,
    resource_link: Option<String>
}
// diesel::table! {
//     lessons (id) {
//         id -> Int4,
//         course_id -> Nullable<Int4>,
//         #[max_length = 255]
//         name -> Varchar,
//         description -> Nullable<Text>,
//         resource_link -> Nullable<Text>,
//     }
// }