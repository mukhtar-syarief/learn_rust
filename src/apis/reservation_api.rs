use actix_web::{ web::{self, Json}, get, Responder};
use diesel::PgConnection;

use crate::{
    repos::{reservation_repo::{
        get_user_reservation
    }, 
    users_repo::get_user_by_username, 
    },
    database::establish_connection, 
    models::{
        reservation::Reservation, 
        users::Users
    },
};


#[get("/{username}")]
pub async fn get_reservation(username: web::Path<String>) -> actix_web::Result<impl Responder> {
    let conn: &mut PgConnection = &mut establish_connection();
    let user: Users = get_user_by_username(conn, &username);

    let reservations: Vec<Reservation> = get_user_reservation(conn, &user.id);
    Ok(Json(reservations))
}
