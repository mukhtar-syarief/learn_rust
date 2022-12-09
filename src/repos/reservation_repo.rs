use diesel::prelude::*;

use crate::schema::reservations::dsl::*;
use crate::schema::reservations;
use crate::models::reservation::{Reservation, NewReservation};

pub fn get_user_reservation(conn: &mut PgConnection, id_user: &i32) -> Vec<Reservation> {
    reservations.filter(user_id.eq(id_user)).load::<Reservation>(conn).expect("Kesalahan pada server.")
}

pub fn create_reservation(conn: &mut PgConnection, reservation: &NewReservation) -> Reservation {

    diesel::insert_into(reservations::table)
        .values(reservation)
        .get_result(conn)
            .expect("Gagal Membuat reservasi.")
}

pub fn delete_reservation_by_id(conn: &mut PgConnection, reservation_id: &i32) {
    diesel::delete(reservations.filter(id.eq(reservation_id)))
        .execute(conn)
            .expect("Gagal menghapus reservasi. Kesalahan pada server.");
}