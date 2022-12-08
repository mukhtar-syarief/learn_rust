use actix_web::{HttpServer, App, web::{self}};
use actix_cors::Cors;

pub mod database;
pub mod repos;
pub mod models;
pub mod schema;
pub mod apis;

use crate::apis::{
    user::{
        create_user, 
        get_user, 
        get_all_user, 
        update_user, 
        delete_this_user,
    },
    cars_api::{
        get_all_cars_type, 
        create_car_type, 
        delete_car_type,
    },
    location_api::{
        get_locations,
        create_location,
        delete_loc,
        get_loc,
    },
    reservation_api::{
        get_reservation
    }
};

async fn tets_api() -> String {
    format!("Hush Bajingan.!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Running in http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                )
            .route("/", web::get().to(tets_api))
            .service(
                web::scope("/user")
                    .service(create_user)
                    .service(get_user)
                    .service(get_all_user)
                    .service(update_user)
                    .service(delete_this_user)
                )
            .service(
                web::scope("/cars")
                    .service(get_all_cars_type)
                    .service(create_car_type)
                    .service(delete_car_type)
                )   
            .service(
                web::scope("/location")
                    .service(get_locations)
                    .service(create_location) 
                    .service(delete_loc) 
                    .service(get_loc) 
            )
            .service(
                web::scope("/reservation")
                    .service(get_reservation)
            )
            }
        )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}