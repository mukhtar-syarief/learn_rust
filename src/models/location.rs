use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

use crate::schema::locations;

#[derive(Queryable)]
#[derive(Serialize)]
#[derive(ToSchema)]
pub struct Location {
    pub id: i32,
    pub region: String
}


#[derive(Serialize)]
#[derive(Insertable)]
#[diesel(table_name = locations)]
pub struct Newlocation<'a> {
    pub region: &'a str
}