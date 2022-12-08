use actix_web::{
    web::{self, Json}, 
    get,
    post,
    delete,
    Responder
};
use serde::{Deserialize, Serialize};

use crate::repos::location_repo::{get_all_location, create_new_loc, delete_location, get_location};
use crate::database::establish_connection;

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
    let locs = get_all_location(conn);
    Ok(Json(locs))
}

#[post("")]
pub async fn create_location(payload: Json<NewLoc>) -> actix_web::Result<impl Responder> {
    let conn = &mut establish_connection();
    let loc = create_new_loc(conn, &payload.region);
    Ok(Json(loc))
}

#[get("/{id}")]
pub async fn get_loc(id: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let conn = &mut establish_connection();
    let loc = get_location(conn, &id);
    Ok(Json(loc))
}


#[delete("/{id}")]
pub async fn delete_loc(id: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let conn = &mut establish_connection();
    delete_location(conn, &id);
    Ok(Json(MessageResponse{
        message: "Lokasi berhasil dihapus".to_string()
    }))
}