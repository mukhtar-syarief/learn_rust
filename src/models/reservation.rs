use diesel::prelude::*;
use chrono::{NaiveDateTime};
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

#[derive(Insertable)]
#[diesel(table_name = reservations)]
pub struct NewReservation<'a> {
    pub vehicle_type_id: &'a i32,
    pub region_id: &'a i32,
    pub user_id: &'a i32,
    pub pickup_date: &'a NaiveDateTime,
    pub return_date: &'a NaiveDateTime
}
