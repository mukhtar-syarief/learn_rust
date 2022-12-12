use actix_web::{
    Responder,
    web::{self, Json, Path},
    get,
    post,
    put,
    delete
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    models::{
        return_reservation::{
            NewReturnReservation,
        }
    },
    database::establish_connection,
    repos::{
        users_repo::UserRepo,
        return_reservation_repo::ReturnReservationRepo,
    },
};


#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    message: String
}

#[derive(Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct InvoicePayload {
    pub user_id: i32,
    pub region_id: i32,
    pub odometer: i32,
    pub full_tank: bool,
    pub time: i64
}

#[derive(Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct UserInvoice {
    pub username: String,
    pub invoice_id: i32
}

#[utoipa::path(
    get,
    path = "/invoice/{username}",
    operation_id = "Get All Invoice",
    responses (
        (
            status = 200,
            description = "Succes",
            body = [ReturnReservation]
        ),
        (
            status = 404,
            description = "Failed"
        )
    ),
    params(
        (
            "username",
            description = "To identify that invoice is right user owned."
        )
    ),
    tag = "Invoice API",
)]
#[get("/{username}")]
pub async fn get_return_reservations(username: Path<String>) -> impl Responder {
    let conn = &mut establish_connection();
    let user = UserRepo::get_user_by_username(conn, &username);
    let invoices = ReturnReservationRepo::get_invoices(conn, &user.id);
    Json(invoices)
}

#[utoipa::path(
    post,
    path = "/invoice/{username}",
    operation_id = "Create Invoice",
    responses (
        (
            status = 200,
            description = "Succes",
            body = ReturnReservation
        ),
        (
            status = 404,
            description = "Failed"
        )
    ),
    params(
        (
            "username",
            description = "To identify that invoice is gived to right user."
        )
    ),
    tag = "Invoice API",
)]
#[post("/{username}")]
pub async fn create_return_reservation(username: Path<String>, payload: Json<InvoicePayload>) -> impl Responder {
    let conn = &mut establish_connection();
    let user = UserRepo::get_user_by_username(conn, &username);

    let invoice_time = NaiveDateTime::from_timestamp_millis(payload.time).unwrap().date();

    let new_invoice = NewReturnReservation {
        full_tank: &payload.full_tank,
        odometer: &payload.odometer,
        region_id: &payload.region_id,
        user_id: &user.id,
        time: Some(&invoice_time),
    };
    let invoice = ReturnReservationRepo::create_invoice(conn, &new_invoice);
    Json(invoice)
}


#[utoipa::path(
    get,
    path = "/invoice/{username}/{invoice_id}",
    operation_id = "Get Invoice",
    responses (
        (
            status = 200,
            description = "Succes",
            body = ReturnReservation
        ),
        (
            status = 404,
            description = "Failed"
        )
    ),
    params(
        (
            "username",
            description = "To identify that invoice is right user owned."
        ),
        (
            "invoice_id",
            description = "To get specify of invoice from user want to see."
        )
    ),
    tag = "Invoice API",
)]
#[get("/{username}/{invoice_id}")]
pub async fn get_return_reservation(path: Path<UserInvoice>) -> impl Responder {
    let conn = &mut establish_connection();
    let user = UserRepo::get_user_by_username(conn, &path.username);
    let invoice = ReturnReservationRepo::get_invoice(conn, &path.invoice_id, &user.id);
    Json(invoice)
}


#[utoipa::path(
    put,
    path = "/invoice/{username}/{invoice_id}",
    operation_id = "Edit Invoice",
    responses (
        (
            status = 200,
            description = "Succes",
            body = MessageResponse
        ),
        (
            status = 404,
            description = "Failed"
        )
    ),
    params(
        (
            "username",
            description = "To identify that invoice is right user owned."
        ),
        (
            "invoice_id",
            description = "To get specify of invoice from user want to edit."
        )
    ),
    tag = "Invoice API",
)]
#[put("/{username}/{invoice_id}")]
pub async fn edit_return_reservation(path: Path<UserInvoice>, payload: Json<InvoicePayload>) -> impl Responder {
    let conn = &mut establish_connection();
    UserRepo::get_user_by_username(conn, &path.username);

    let new_invoice = NewReturnReservation {
        full_tank: &payload.full_tank,
        odometer: &payload.odometer,
        region_id: &payload.region_id,
        user_id: &payload.user_id,
        time: None
    };
    
    ReturnReservationRepo::edit_invoice(conn, &new_invoice);
    
    Json(
        MessageResponse {
            message: "Update invoice berhasil.".to_string()
        }
    )
}


#[utoipa::path(
    delete,
    path = "/invoice/{username}/{invoice_id}",
    operation_id = "Delete invoice",
    responses (
        (
            status = 200,
            description = "Succes",
            body = MessageResponse
        ),
        (
            status = 404,
            description = "Failed"
        )
    ),
    params(
        (
            "username",
            description = "To identify that invoice is right user owned."
        ),
        (
            "invoice_id",
            description = "To get specify of invoice from user want to remove."
        )
    ),
    tag = "Invoice API",
)]
#[delete("/{username}/{invoice_id}")]
pub async fn delete_return_reservation(path: Path<UserInvoice>) -> impl Responder {
    let conn = &mut establish_connection();
    let user = UserRepo::get_user_by_username(conn, &path.username);

    ReturnReservationRepo::delete_invoice(conn, &path.invoice_id, &user.id);
    Json(
        MessageResponse {
            message: "Berhasil dihapus.".to_string()
        }
    )
}

pub fn return_reservation_api_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/invoice")
            .service(create_return_reservation)
            .service(get_return_reservations)
            .service(get_return_reservation)
            .service(edit_return_reservation)
            .service(delete_return_reservation)
    );
}