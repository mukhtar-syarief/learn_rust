use actix_web::{get, post, web, Responder};
use serde::{Serialize, Deserialize};
// use std::future::Future;
// use futures_util::stream::StreamExt as _;

use crate::database::establish_connection;
// use crate::models::users::Users;
use crate::repos::users_repo::{get_all, get_user_by_username, create_new_user};
// use crate::database::establish_connection;


#[derive(Serialize, Deserialize)]
pub struct UserAuth {
    pub username: String,
    pub password: String
}

#[get("")]
pub async fn get_all_user() -> actix_web::Result<impl Responder> {
    let conn = &mut establish_connection();
    let users = get_all(conn).unwrap();
    Ok(web::Json(users))
}

#[post("")]
pub async fn create_user(payload: web::Json<UserAuth>) -> actix_web::Result<impl Responder>{
    let conn = &mut establish_connection();
    println!("asd");
    let user = create_new_user(conn, &payload.username, &payload.password);
    Ok(web::Json(user))
}

#[get("/{username}")]
pub async fn get_user(username: web::Path<String>) -> actix_web::Result<impl Responder> {
    let conn = &mut establish_connection();
    let user = get_user_by_username(conn, &username).unwrap();
    // let user = UserAuth {
    //     password: "Hallo cah..!".to_string(),
    //     username: username.to_string()
    // };
    Ok(web::Json(user))
} 
