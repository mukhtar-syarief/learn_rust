use actix_web::{ get, post, delete, Responder, web::{self, Json}};
use diesel::PgConnection;
use serde::{Serialize, Deserialize};

use crate::{
    database::establish_connection,
    models::cars::Cars,
};


#[derive(Deserialize, Serialize)]
pub struct CarPayload {
    pub type_: String
}

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    message: String
} 

#[get("")]
pub async fn get_all_cars_type() -> actix_web::Result<impl Responder>{
    let conn: &mut PgConnection = &mut establish_connection();
    let cars: Vec<Cars> = Cars::get_all_type(conn);
    Ok(Json(cars))
}

#[post("")]
pub async fn create_car_type(payload: Json<CarPayload>) -> actix_web::Result<impl Responder> {
    let conn: &mut PgConnection = &mut establish_connection();
    let car: Cars = Cars::create_new_car(conn, &payload.type_);

    Ok(Json(car))
}

#[delete("/{type}")]
pub async fn delete_car_type(type_: web::Path<String>) -> actix_web::Result<impl Responder>{
    let conn: &mut PgConnection = &mut establish_connection();
    Cars::delete_type(conn, &type_);
    Ok(Json(MessageResponse{
        message: "Tipe mobil dihapus".to_string()
    }))
}