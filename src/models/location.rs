use diesel::prelude::*;
use serde::Serialize;

use crate::schema::locations;

#[derive(Queryable)]
#[derive(Serialize)]
pub struct Location {
    pub id: i32,
    pub region: String
}


#[derive(Insertable)]
#[diesel(table_name = locations)]
pub struct Newlocation<'a> {
    pub region: &'a str
}