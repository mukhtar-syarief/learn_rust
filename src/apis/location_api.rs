use actix_web::{
    web::{self, Json}, 
    get,
    post,
    delete,
    Responder
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::database::establish_connection;
use crate::repos::location_repo::LocationRepo;

#[derive(Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct NewLoc {
    pub region: String
}

#[derive(Serialize, Deserialize)]
pub struct MessageResponse {
    pub message: String
} 


#[utoipa::path(
    get,
    path = "/location",
    operation_id = "Get All Location",
    responses(
        (status = 200, description = "Success get all location.", body = [Location]),
        (status = 404, description = "Trouble with API.")
    ),
    tag = "Location API",
)]
#[get("")]
pub async fn get_locations() -> actix_web::Result<impl Responder>{
    let conn = &mut establish_connection();
    let locs = LocationRepo::get_all_location(conn);
    Ok(Json(locs))
}


#[utoipa::path(
    post,
    path = "/location",
    operation_id = "Create Location",
    responses(
        (status = 200, description = "Success create new location.", body = Location),
        (status = 404, description = "Trouble with API.")
    ),
    request_body = NewLoc,
    tag = "Location API",
)]
#[post("")]
pub async fn create_location(payload: Json<NewLoc>) -> actix_web::Result<impl Responder> {
    let conn = &mut establish_connection();
    let loc = LocationRepo::create_new_loc(conn, &payload.region);
    Ok(Json(loc))
}

#[utoipa::path(
    get,
    path = "/location/{id}",
    operation_id = "Get Location",
    responses(
        (status = 200, description = "Success get location.", body = Location),
        (status = 404, description = "Trouble with API.")
    ),
    params(
        ("id", description = "id of location you want to see.")
    ),
    tag = "Location API",
)]
#[get("/{id}")]
pub async fn get_loc(id: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let conn = &mut establish_connection();
    let loc = LocationRepo::get_location(conn, &id);
    Ok(Json(loc))
}

#[utoipa::path(
    delete,
    path = "/location/{id}",
    operation_id = "Delete Location",
    responses(
        (status = 200, description = "Success delete location.", body = MessageResponse),
        (status = 404, description = "Trouble with API.")
    ),
    params(
        ("id", description = "id of location you want to delete.")
    ),
    tag = "Location API",
)]
#[delete("/{id}")]
pub async fn delete_loc(id: web::Path<i32>) -> actix_web::Result<impl Responder> {
    let conn = &mut establish_connection();
    LocationRepo::delete_location(conn, &id);
    Ok(Json(MessageResponse{
        message: "Lokasi berhasil dihapus".to_string()
    }))
}

pub fn location_api_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/location")
            .service(get_locations)
            .service(create_location) 
            .service(delete_loc) 
            .service(get_loc)
    );
}