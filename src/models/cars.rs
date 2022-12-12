use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use crate::schema::cars;


#[derive(Queryable)]
#[derive(Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct Cars {
    pub id: i32,
    pub type_: String
}

#[derive(Insertable)]
#[derive(Serialize, Deserialize)]
#[diesel(table_name = cars)]
pub struct NewCars<'a> {
    pub type_: &'a str
}