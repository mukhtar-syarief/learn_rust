use chrono::NaiveDate;
use diesel::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;

use crate::schema::return_reservation;

#[derive(Queryable)]
#[derive(Serialize)]
#[derive(ToSchema)]
pub struct ReturnReservation {
    pub id: i32,
    pub user_id: i32,
    pub region_id: i32,
    pub odometer: i32,
    pub full_tank: bool,
    pub time: NaiveDate
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = return_reservation)]
pub struct NewReturnReservation<'a> {
    pub user_id: &'a i32,
    pub region_id: &'a i32,
    pub odometer: &'a i32,
    pub full_tank: &'a bool,
    pub time: Option<&'a NaiveDate>
}