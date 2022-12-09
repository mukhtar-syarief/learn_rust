use diesel::prelude::*;
use chrono::NaiveDateTime;
use serde::Serialize;

use crate::schema::reservations;

#[derive(Queryable)]
#[derive(Serialize)]
pub struct Reservation {
    pub id: i32,
    pub vehicle_type_id: i32,
    pub region_id: i32,
    pub user_id: i32,
    pub pickup_date: NaiveDateTime,
    pub return_date: NaiveDateTime
}

#[derive(Insertable, AsChangeset)]
#[diesel(table_name = reservations)]
pub struct NewReservation<'a> {
    pub vehicle_type_id: Option<&'a i32>,
    pub region_id: Option<&'a i32>,
    pub user_id: Option<&'a i32>,
    pub pickup_date: Option<NaiveDateTime>,
    pub return_date: Option<NaiveDateTime>
}
