use actix_web::{
    Responder,
    {
        web::Json,
        web::Path,
    },
    get,
    post,
    put,
    delete
};
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        return_reservation::{
            NewReturnReservation,
            ReturnReservation
        },
        users::Users,
    },
    database::establish_connection,
};


#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    message: String
}

#[derive(Serialize, Deserialize)]
pub struct InvoicePayload {
    user_id: i32,
    region_id: i32,
    odometer: i32,
    full_tank: bool,
    time: i64
}

#[derive(Serialize, Deserialize)]
pub struct UserInvoice {
    pub username: String,
    pub invoice_id: i32
}


#[get("/{username}")]
pub async fn get_return_reservations(username: Path<String>) -> impl Responder {
    let conn = &mut establish_connection();
    let user = Users::get_user_by_username(conn, &username);
    let invoices = ReturnReservation::get_invoices(conn, &user.id);
    Json(invoices)
}

#[post("/{username}")]
pub async fn create_return_reservation(username: Path<String>, payload: Json<InvoicePayload>) -> impl Responder {
    let conn = &mut establish_connection();
    let user = Users::get_user_by_username(conn, &username);


    let new_invoice = NewReturnReservation {
        full_tank: &payload.full_tank,
        odometer: &payload.odometer,
        region_id: &payload.region_id,
        user_id: &user.id,
        time: None
    };
    let invoice = ReturnReservation::create_invoice(conn, &new_invoice);
    Json(invoice)
}

#[get("/{username}/{invoice_id}")]
pub async fn get_return_reservation(path: Path<UserInvoice>) -> impl Responder {
    let conn = &mut establish_connection();
    let user = Users::get_user_by_username(conn, &path.username);
    let invoice = ReturnReservation::get_invoice(conn, &path.invoice_id, &user.id);
    Json(invoice)
}

#[put("/{username}/{invoice_id}")]
pub async fn edit_return_reservation(path: Path<UserInvoice>, payload: Json<InvoicePayload>) -> impl Responder {
    let conn = &mut establish_connection();
    Users::get_user_by_username(conn, &path.username);

    let new_invoice = NewReturnReservation {
        full_tank: &payload.full_tank,
        odometer: &payload.odometer,
        region_id: &payload.region_id,
        user_id: &payload.user_id,
        time: None
    };
    
    ReturnReservation::edit_invoice(conn, &new_invoice);
    
    Json(
        MessageResponse {
            message: "Update invoice berhasil.".to_string()
        }
    )
}

#[delete("/{username}/{invoice_id}")]
pub async fn delete_return_reservation(path: Path<UserInvoice>) -> impl Responder {
    let conn = &mut establish_connection();
    let user = Users::get_user_by_username(conn, &path.username);

    ReturnReservation::delete_invoice(conn, &path.invoice_id, &user.id);
    Json(
        MessageResponse {
            message: "Berhasil dihapus.".to_string()
        }
    )
}