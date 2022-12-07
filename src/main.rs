use actix_web::{HttpServer, App, web::{self}};

pub mod database;
pub mod repos;
pub mod models;
pub mod schema;
pub mod apis;

async fn tets_api() -> String {
    format!("Hush Bajingan.!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use crate::apis::user::{create_user, get_user, get_all_user};

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(tets_api))
            .service(
                web::scope("/user")
                        .service(create_user)
                        .service(get_user)
                        .service(get_all_user)
                    )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}