use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Debug, Serialize, Deserialize, Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String
}

#[derive(Identifiable, Queryable, Serialize, Deserialize, Associations, Clone)]
#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub user_id: Option<i32>
}

// #[derive(Deserialize, Insertable)]
// #[table_name = "posts"]
// pub struct PostNew<'a> {
//     pub title: &'a str,
//     pub user_id: Option<i32>
// }

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name = "posts"]
pub struct PostCreateJson {
    pub title: String,
    pub user_id: Option<i32>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostUpdateJson {
    pub id: i32,
    pub title: String,
    pub user_id: Option<i32>
}

#[derive(Debug, Insertable)]
#[table_name = "users"]
pub struct UserNew<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub created_at: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserJson {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateJson {
    pub id: i32,
    pub name: String,
    pub email: String,
}