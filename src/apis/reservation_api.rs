use actix_web::{ web::{self, Json}, get, post, Responder, put, delete};
use diesel::PgConnection;
use serde::{Serialize, Deserialize};
use chrono::{NaiveDateTime};
use diesel::prelude::*;
use utoipa::ToSchema;

use crate::{
    database::establish_connection, 
    models::{
        reservation::{Reservation, NewReservation}, 
        users::Users
    },
    schema::reservations::dsl::*,
    repos::{
        reservation_repo::ReservationRepo,
        users_repo::UserRepo,
    },
};

#[derive(Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct MessageResponse {
    message: String
} 

#[derive(Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct ReservationPayload {
    pub vehicle_type_id: i32,
    pub region_id: i32,
    pub pickup_date: i64,
    pub return_date: i64
}


#[utoipa::path(
    get,
    path = "/reservation/{username}",
    operation_id = "Get All Reservation",
    responses(
        (
            status = 200,
            description = "Success",
            body = [Reservation]
        ),
        (
            status = 404,
            description = "Fail"
        )
    ),
    params(
        (
            "username",
            description = "username of user to check reservation baser user."
        )
    ),
    tag = "Reservation API",
)]
#[get("/{username}")]
pub async fn get_reservations(username: web::Path<String>) -> actix_web::Result<impl Responder> {
    let conn: &mut PgConnection = &mut establish_connection();
    let user: Users = UserRepo::get_user_by_username(conn, &username);

    let user_reservation: Vec<Reservation> = ReservationRepo::get_user_reservation(conn, &user.id);
    Ok(Json(user_reservation))
}


#[utoipa::path(
    post,
    path = "/reservation/{username}",
    operation_id = "Create Reservation",
    responses(
        (
            status = 200,
            description = "Success",
            body = Reservation
        ),
        (
            status = 404,
            description = "Fail"
        )
    ),
    params(
        (
            "username",
            description = "username of user to check reservation baser user."
        )
    ),
    request_body = ReservationPayload,
    tag = "Reservation API",
)]
#[post("/{username}")]
pub async fn create_user_reservation(username: web::Path<String>, payload: Json<ReservationPayload>) -> actix_web::Result<impl Responder> {
    let conn = &mut establish_connection();
    
    let user_pickup = NaiveDateTime::from_timestamp_millis(payload.pickup_date).unwrap();
    let user_return = NaiveDateTime::from_timestamp_millis(payload.return_date).unwrap();

    let user = UserRepo::get_user_by_username(conn, &username);

    let new_reservation = NewReservation {
        pickup_date: Some(user_pickup),
        return_date: Some(user_return),
        region_id: Some(&payload.region_id),
        user_id: Some(&user.id),
        vehicle_type_id: Some(&payload.vehicle_type_id)
    };
    let result = ReservationRepo::create_reservation(conn, &new_reservation);
    Ok(Json(result))
}


#[derive(Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct UserReservation {
    pub username: String,
    pub reservation_id: i32
}


#[utoipa::path(
    get,
    path = "/reservation/{username}/{reservation_id}",
    operation_id = "Get Reservation",
    responses(
        (
            status = 200,
            description = "Success",
            body = Reservation
        ),
        (
            status = 404,
            description = "Fail"
        )
    ),
    params(
        (
            "username",
            description = "username of user to check reservation baser user."
        ),
        (
            "reservation_id",
            description = "id for get specific of reservation."
        )
    ),
    tag = "Reservation API",
)]
#[get("/{username}/{reservation_id}")]
pub async fn get_reservation(
    path: web::Path<UserReservation>
) -> actix_web::Result<impl Responder> {
    let conn = &mut establish_connection();
    UserRepo::get_user_by_username(conn, &path.username);

    let reservation = reservations.filter(id.eq(&path.reservation_id))
        .first::<Reservation>(conn).expect("Kesalahan pada server");
    
    Ok(Json(reservation))
}

#[utoipa::path(
    put,
    path = "/reservation/{username}/{reservation_id}",
    operation_id = "Edit Reservation",
    responses(
        (
            status = 200,
            description = "Success",
            body = MessageResponse
        ),
        (
            status = 404,
            description = "Fail"
        )
    ),
    params(
        (
            "username",
            description = "username of user to check reservation baser user."
        ),
        (
            "reservation_id",
            description = "id for get specific of reservation."
        )
    ),
    request_body = ReservationPayload,
    tag = "Reservation API",
)]
#[put("/{username}/{reservation_id}")]
pub async fn edit_this_reservation(
    path: web::Path<UserReservation>,
    payload: Json<ReservationPayload>
) -> actix_web::Result<impl Responder> {

    let conn = &mut establish_connection();

    UserRepo::get_user_by_username(conn, &path.username);

    let user_pickup = NaiveDateTime::from_timestamp_millis(payload.pickup_date);
    let user_return = NaiveDateTime::from_timestamp_millis(payload.return_date);

    let new_reservation = NewReservation {
        pickup_date: user_pickup,
        return_date: user_return,
        region_id: Some(&payload.region_id),
        user_id: None,
        vehicle_type_id: Some(&payload.vehicle_type_id)
    };

    diesel::update(reservations.find(&path.reservation_id))
            .set(&new_reservation)
            .execute(conn).unwrap();
    Ok(Json(
        MessageResponse {
            message: "Reservasi Berhasil diperbaharui.".to_string()
        }
    ))
}

#[utoipa::path(
    delete,
    path = "/reservation/{username}/{reservation_id}",
    operation_id = "Delete Reservation",
    responses(
        (
            status = 200,
            description = "Success",
            body = Reservation
        ),
        (
            status = 404,
            description = "Fail"
        )
    ),
    params(
        (
            "username",
            description = "username of user to check reservation baser user."
        ),
        (
            "reservation_id",
            description = "id for get specific of reservation."
        )
    ),
    tag = "Reservation API",
)]
#[delete("/{username}/{reservation_id}")]
pub async fn delete_this_reservation(path: web::Path<UserReservation>) -> impl Responder{
    let conn = &mut establish_connection();
    ReservationRepo::delete_reservation_by_id(conn, &path.reservation_id);
    Json(
        MessageResponse {
            message: "Reservasi dihapus.".to_string()
        }
    )
}   