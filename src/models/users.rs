use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

use crate::schema::users;


#[derive(Queryable)]
#[derive(Serialize)]
#[derive(ToSchema)]
#[derive(Debug)]
pub struct Users {
    pub id: i32,
    pub username: String,
    pub password: String,
}

#[derive(Insertable)]
#[derive(Serialize)]
#[diesel(table_name = users)]
pub struct NewUser <'a> {
    pub username: &'a str,
    pub password: &'a str
}