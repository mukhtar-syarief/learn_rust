use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

use crate::schema::cars;


#[derive(Queryable)]
#[derive(Serialize)]
#[derive(ToSchema)]
pub struct Cars {
    pub id: i32,
    pub type_: String
}

#[derive(Insertable)]
#[diesel(table_name = cars)]
pub struct NewCars<'a> {
    pub type_: &'a str
}