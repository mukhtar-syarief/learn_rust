use actix_web::{
    web::{self, Json}, 
    get,
    post,
    delete,
    Responder
};
use serde::{Deserialize, Serialize};

use crate::database::establish_connection;
use crate::models::location::Location;

#[derive(Serialize, Deserialize)]
pub struct NewLoc {
    region: String
}

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    message: String
} 


#[get("")]
pub async fn get_locations() -> actix_web::Result<impl Responder>{
    let conn = &mut establish_connection();
    let locs = Location::get_all_location(conn);
    Ok(Json(locs))
}

#[post("")]
pub async fn create_location(payload: Json<NewLoc>) -> actix_web::Result<impl Responder> {
    let conn = &mut establish_connection();
    let loc = Location::create_new_loc(conn, &payload.region);
    Ok(Json(loc))
}

#[get("/{id}")]
pub async fn get_loc(id: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let conn = &mut establish_connection();
    let loc = Location::get_location(conn, &id);
    Ok(Json(loc))
}


#[delete("/{id}")]
pub async fn delete_loc(id: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let conn = &mut establish_connection();
    Location::delete_location(conn, &id);
    Ok(Json(MessageResponse{
        message: "Lokasi berhasil dihapus".to_string()
    }))
}