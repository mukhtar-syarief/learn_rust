use actix_web::{test, App};
use diesel::prelude::*;
use learn_basic_rust as root;

use root::{
    apis::user::users_api_services, 
    models::users::{NewUser, Users},
    database::establish_connection,
    schema::{
        users::dsl::*,
    },
};

#[actix_web::test]
pub async fn test_get_all_user(){
    let app = test::init_service(
        App::new()
            .configure(users_api_services)
    )
    .await;
    let req = test::TestRequest::get().uri("/user").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
pub async fn test_create_user() {
    let conn = &mut establish_connection();
    diesel::delete(users.filter(username.eq("test"))).execute(conn).unwrap();
    let app = test::init_service(
        App::new()
        .configure(users_api_services)
    )
    .await;
    let user_papyload = NewUser {
        username: "test",
        password: "test"
    };

    let req = test::TestRequest::post().uri("/user").set_json(&user_papyload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    diesel::delete(users.filter(username.eq("test"))).execute(conn).unwrap();
}

#[actix_web::test]
pub async fn test_get_user(){
    let conn = &mut establish_connection();
    diesel::delete(users.filter(username.eq("test"))).execute(conn).unwrap();

    let app = test::init_service(
        App::new()
            .configure(users_api_services)
    )
    .await;
    
    // Create User
    let user_papyload = NewUser {
        username: "test",
        password: "test"
    };
    let req = test::TestRequest::post().uri("/user").set_json(&user_papyload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Test get user
    let req = test::TestRequest::get().uri("/user/test").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    diesel::delete(users.filter(username.eq("test"))).execute(conn).unwrap();
    let user = users.load::<Users>(conn).unwrap();
    assert_eq!(0, user.len());
    
}

#[actix_web::test]
pub async fn test_edit_user(){
    let conn = &mut establish_connection();
    diesel::delete(users.filter(username.eq("test"))).execute(conn).unwrap();

    let app = test::init_service(
        App::new()
            .configure(users_api_services)
    )
    .await;
    
    // Create User
    let user_papyload = NewUser {
        username: "test",
        password: "test"
    };
    let req = test::TestRequest::post().uri("/user").set_json(&user_papyload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // edit
    let user_edit = NewUser {
        username: "test",
        password: "hahahaha"
    };
    let req = test::TestRequest::put().uri("/user/test").set_json(&user_edit).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    let user = users.filter(username.eq("test")).first::<Users>(conn).unwrap();
    assert_eq!("hahahaha", user.password);

    // Test get user
    let req = test::TestRequest::get().uri("/user/test").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    
    diesel::delete(users.filter(username.eq("test"))).execute(conn).unwrap();
    let user = users.load::<Users>(conn).unwrap();
    assert_eq!(0, user.len());
}

#[actix_web::test]
pub async fn test_delete_user(){
    let conn = &mut establish_connection();
    diesel::delete(users.filter(username.eq("test"))).execute(conn).unwrap();

    let app = test::init_service(
        App::new()
            .configure(users_api_services)
    )
    .await;
    
    // Create User
    let user_papyload = NewUser {
        username: "test",
        password: "test"
    };
    let req = test::TestRequest::post().uri("/user").set_json(&user_papyload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Test get user
    let req = test::TestRequest::get().uri("/user/test").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // delete
    let req = test::TestRequest::delete().uri("/user/test").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}