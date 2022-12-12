use actix_web::{ get, post, delete, Responder, web::{self, Json}};
use diesel::PgConnection;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use crate::{
    database::establish_connection,
    models::cars::Cars,
    repos::cars_repo::CarsRepo
};


#[derive(Deserialize, Serialize)]
#[derive(ToSchema)]
pub struct CarPayload {
    pub type_: String
}

#[derive(Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct MessageResponse {
    message: String
} 

#[utoipa::path(
    get,
    path = "/cars",
    operation_id = "Get All Car Type",
    responses (
        (status = 200, description = "API to get all type of Car.", body = [Cars]),
        (status = 404, description = "It's problem from API.")
    ),
    tag = "Cars API", 
)]
#[get("")]
pub async fn get_all_cars_type() -> actix_web::Result<impl Responder>{
    let conn: &mut PgConnection = &mut establish_connection();
    let cars: Vec<Cars> = CarsRepo::get_all_type(conn);
    Ok(Json(cars))
}

#[utoipa::path(
    post,
    path = "/cars",
    operation_id = "Crate Car Type",
    responses (
        (status = 200, description = "API to create new type of Car.", body = Cars),
        (status = 404, description = "It's problem from API.")
    ),
    request_body = CarPayload,
    tag = "Cars API", 
)]
#[post("")]
pub async fn create_car_type(payload: Json<CarPayload>) -> actix_web::Result<impl Responder> {
    let conn: &mut PgConnection = &mut establish_connection();
    let car: Cars = CarsRepo::create_new_car(conn, &payload.type_);

    Ok(Json(car))
}

#[utoipa::path(
    delete,
    path = "/cars/{type}",
    operation_id = "Delete Car Type",
    responses (
        (status = 200, description = "API to get all type of Car.", body = MessageResponse),
        (status = 404, description = "It's problem from API.")
    ),
    params(
        ("type", description = "Type of car you want to delete.")
    ),
    tag = "Cars API", 
)]
#[delete("/{type}")]
pub async fn delete_car_type(type_: web::Path<String>) -> actix_web::Result<impl Responder>{
    let conn: &mut PgConnection = &mut establish_connection();
    CarsRepo::delete_type(conn, &type_);
    Ok(Json(MessageResponse{
        message: "Tipe mobil dihapus".to_string()
    }))
}


pub fn cars_api_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/cars")
        .service(get_all_cars_type)
        .service(create_car_type)
        .service(delete_car_type)
    );
}