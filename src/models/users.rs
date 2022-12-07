use diesel::prelude::*;
use serde::Serialize;

use crate::schema::users;


#[derive(Queryable)]
#[derive(Serialize)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name =users)]
pub struct NewUser <'a> {
    pub username: &'a str,
    pub password: &'a str
}