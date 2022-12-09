use diesel::prelude::*;

use crate::{
    schema::return_reservation::dsl::*,
    schema::return_reservation,
    models::return_reservation::{ReturnReservation, NewReturnReservation},
};


pub fn get_invoices(conn: &mut PgConnection, id_user: &i32) -> Vec<ReturnReservation> {
    let invoices = return_reservation.filter(user_id.eq(id_user))
        .load(conn)
            .expect("Kesalahan pada server");
    invoices
}

pub fn get_invoice(conn: &mut PgConnection, invoice_id: &i32, id_user: &i32) -> ReturnReservation {
    return_reservation.find(invoice_id)
        .filter(user_id.eq(id_user))
        .first(conn)
            .expect("Kesalahan Tidak diketahui")
}

pub fn edit_invoice(conn: &mut PgConnection, invoice: &NewReturnReservation) {
    diesel::update(return_reservation.find(invoice.user_id))
        .set(invoice)
        .execute(conn)
            .expect("Gagal update invoice.");
}

pub fn delete_invoice(conn: &mut PgConnection, invoice_id: &i32, id_user: &i32){
    diesel::delete(return_reservation.filter(id.eq(invoice_id)).filter(user_id.eq(id_user)))
        .execute(conn)
            .expect("Gagal menghapus invoice.");
}

pub fn create_invoice(conn: &mut PgConnection, invoice: &NewReturnReservation) -> ReturnReservation {
    diesel::insert_into(return_reservation::table)
        .values(invoice)
            .get_result(conn)
                .expect("Invoice gagal dibuat.")
}
