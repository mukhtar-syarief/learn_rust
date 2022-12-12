use actix_web::{HttpServer, App, web, Result, HttpResponse, http::StatusCode};
use actix_cors::Cors;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use learn_basic_rust as root;


use root::documentation::ApiDoc;
use root::apis::{
    user::users_api_services,
    cars_api::cars_api_services,
    location_api::location_api_service,
    reservation_api::reservation_api_services,
    return_reservation_api::return_reservation_api_service,
};

async fn test_api() -> String {
    format!("Hush Bajingan.!")
}


async fn redoc() -> Result<HttpResponse> {
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(include_str!("static/redoc.html")))
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
            .route("/", web::get().to(test_api))
            .configure(users_api_services)
            .configure(cars_api_services)   
            .configure(location_api_service)
            .configure(reservation_api_services)
            .configure(return_reservation_api_service)
            .route(
                "/redoc", 
                web::get().to(redoc)
            )
            .service(
                SwaggerUi::new("/docs/{_:.*}")
                .url("/api-doc/openapi.json", ApiDoc::openapi()),
            )
            }
        )
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}