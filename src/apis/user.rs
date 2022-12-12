use actix_web::{get, post, put, delete, web, Responder};
use diesel::PgConnection;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use crate::database::establish_connection;
use crate::models::users::Users;
use crate::repos::users_repo::UserRepo;

#[derive(Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct MessageResponse {
    message: String
} 

#[derive(Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct UserAuth {
    pub username: String,
    pub password: String
}


#[utoipa::path(
    get,
    path = "/user",
    operation_id= "Get All User",
    responses(
        (status = 200, description = "Success get all user.", body = [Users]),
        (status = 404, description = "Trouble with API.")
    ),
    tag = "User API"
)]
#[get("")]
pub async fn get_all_user() -> actix_web::Result<impl Responder> {
    let conn:&mut PgConnection = &mut establish_connection();
    let users: Vec<Users> = UserRepo::get_all(conn);

    Ok(web::Json(users))
}



#[utoipa::path(
    post,
    path = "/user",
    operation_id = "Create User",
    responses(
        (status = 200, description = "Use this path to create user.", body = Users),
        (status = 404, description = "Trouble with API.")
    ),
    request_body = UserAuth,
    tag = "User API"
)]
#[post("")]
pub async fn create_user(payload: web::Json<UserAuth>) -> actix_web::Result<impl Responder>{
    let conn:&mut PgConnection = &mut establish_connection();
    let username: String = String::from(payload.username.split_whitespace().collect::<String>().to_lowercase());
    let user:Users = UserRepo::create_new_user(conn, &username, &payload.password);

    Ok(web::Json(user))
}


#[utoipa::path(
    get,
    path = "/user/{username}",
    operation_id = "Get User",
    responses(
        (status = 200, description = "Use this path to get user.", body = Users),
        (status = 404, description = "Trouble with API.")
    ),
    params(
        ("username", description="username of user to indentify.")
    ),
    tag = "User API"
)]
#[get("/{username}")]
pub async fn get_user(username: web::Path<String>) -> actix_web::Result<impl Responder> {
    let conn:&mut PgConnection = &mut establish_connection();
    let user:Users = UserRepo::get_user_by_username(conn, &username);

    Ok(web::Json(user))
} 


#[utoipa::path(
    put,
    path = "/user/{username}",
    operation_id = "Edit User",
    responses(
        (status = 200, description = "Use this path to edit user.", body = MessageResponse),
        (status = 404, description = "Trouble with API.")
    ),
    params(
        ("username", description="username of user to indentify.")
    ),
    tag = "User API"
)]
#[put("/{username}")]
pub async fn update_user(username: web::Path<String>, payload: web::Json<UserAuth>) -> actix_web::Result<web::Json<MessageResponse>>{
    let conn:&mut PgConnection =&mut establish_connection();
    let user:Users = UserRepo::get_user_by_username(conn, &username);
    let new_username: String = String::from(payload.username.split_whitespace().collect::<String>().to_lowercase());
    UserRepo::edit_user(conn, &user.id , &new_username, &payload.password);

    Ok(web::Json(MessageResponse {
        message: String::from("Update Berhasil")
    }))
}


#[utoipa::path(
    delete,
    path = "/user/{username}",
    operation_id = "Delete User",
    responses(
        (status = 200, description = "Use this path to delete user.", body = MessageResponse),
        (status = 404, description = "Trouble with API.")
    ),
    params(
        ("username", description="username to indentify that user will delete.")
    ),
    tag = "User API"
)]
#[delete("/{username}")]
pub async fn delete_this_user(username: web::Path<String>) -> actix_web::Result<web::Json<MessageResponse>>{
    let conn:&mut PgConnection = &mut establish_connection();
    UserRepo::delete_user(conn, &username);
    Ok(web::Json(
        MessageResponse { message: String::from("User berhasil dihapus") }
    ))
}

pub fn users_api_services(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(create_user)
            .service(get_user)
            .service(get_all_user)
            .service(update_user)
            .service(delete_this_user)
    );
}