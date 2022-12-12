use actix_web::{test, App};
use diesel::prelude::*;

use learn_basic_rust as root;


use root::{
    schema::cars::dsl::*,
    database::establish_connection,
    models::cars::{NewCars},
    apis::cars_api::cars_api_services,
};

#[actix_web::test]
pub async fn test_get_all_car_type() {
    let app = test::init_service(
        App::new()
        .configure(cars_api_services)
    ).await;

    let req = test::TestRequest::get().uri("/cars").to_request();
    let res = test::call_service(&app, req).await;
    assert!(res.status().is_success())
}

#[actix_web::test]
pub async fn test_create_car_type(){
    let conn = &mut establish_connection();

    let app = test::init_service(
        App::new()
        .configure(cars_api_services)
    ).await;

    let payload = NewCars {
        type_: "Test"
    };

    let req = test::TestRequest::post().uri("/cars").set_json(&payload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    diesel::delete(cars.filter(type_.eq(&payload.type_))).execute(conn).unwrap();
}

#[actix_web::test]
pub async fn test_delete_car_type(){

    let app = test::init_service(
        App::new()
        .configure(cars_api_services)
    ).await;

    let payload = NewCars {
        type_: "Test"
    };

    // Create
    let req = test::TestRequest::post().uri("/cars").set_json(&payload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::delete().uri("/cars/Test").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
