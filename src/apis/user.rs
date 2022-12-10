use actix_web::{get, post, put, delete, web, Responder};
use diesel::PgConnection;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;

use crate::database::establish_connection;
use crate::models::users::Users;

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
    responses(
        (status = 200, description = "Success get all user.", body = [Users]),
        (status = 404, description = "Trouble with API.")
    )
)]
#[get("")]
pub async fn get_all_user() -> actix_web::Result<impl Responder> {
    let conn:&mut PgConnection = &mut establish_connection();
    let users: Vec<Users> = Users::get_all(conn);

    Ok(web::Json(users))
}



#[utoipa::path(
    post,
    path = "/user",
    responses(
        (status = 200, description = "Use this path to create user.", body = Users),
        (status = 404, description = "Trouble with API.")
    ),
    request_body = UserAuth
)]
#[post("")]
pub async fn create_user(payload: web::Json<UserAuth>) -> actix_web::Result<impl Responder>{
    let conn:&mut PgConnection = &mut establish_connection();
    let username: String = String::from(payload.username.split_whitespace().collect::<String>().to_lowercase());
    let user:Users = Users::create_new_user(conn, &username, &payload.password);

    Ok(web::Json(user))
}


#[utoipa::path(
    get,
    path = "/user/{username}",
    responses(
        (status = 200, description = "Use this path to get user.", body = Users),
        (status = 404, description = "Trouble with API.")
    ),
    params(
        ("username", description="username of user to indentify.")
    )
)]
#[get("/{username}")]
pub async fn get_user(username: web::Path<String>) -> actix_web::Result<impl Responder> {
    let conn:&mut PgConnection = &mut establish_connection();
    let user:Users = Users::get_user_by_username(conn, &username);

    Ok(web::Json(user))
} 


#[utoipa::path(
    put,
    path = "/user/{username}",
    responses(
        (status = 200, description = "Use this path to edit user.", body = MessageResponse),
        (status = 404, description = "Trouble with API.")
    ),
    params(
        ("username", description="username of user to indentify.")
    )
)]
#[put("/{username}")]
pub async fn update_user(username: web::Path<String>, payload: web::Json<UserAuth>) -> actix_web::Result<web::Json<MessageResponse>>{
    let conn:&mut PgConnection =&mut establish_connection();
    let user:Users = Users::get_user_by_username(conn, &username);
    let new_username: String = String::from(payload.username.split_whitespace().collect::<String>().to_lowercase());
    Users::edit_user(conn, &user.id , &new_username, &payload.password);

    Ok(web::Json(MessageResponse {
        message: String::from("Update Berhasil")
    }))
}


#[utoipa::path(
    delete,
    path = "/user/{username}",
    responses(
        (status = 200, description = "Use this path to delete user.", body = MessageResponse),
        (status = 404, description = "Trouble with API.")
    ),
    params(
        ("username", description="username to indentify that user will delete.")
    )
)]
#[delete("/{username}")]
pub async fn delete_this_user(username: web::Path<String>) -> actix_web::Result<web::Json<MessageResponse>>{
    let conn:&mut PgConnection = &mut establish_connection();
    Users::delete_user(conn, &username);
    Ok(web::Json(
        MessageResponse { message: String::from("User berhasil dihapus") }
    ))
}