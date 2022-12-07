use diesel::prelude::*;

#[derive(Queryable)]
pub struct UserDetail {
    pub id: i32,
    pub fullname: String,
    pub nickname: String,
    pub address: String,
    pub age: i32,
    pub is_adult: bool
}