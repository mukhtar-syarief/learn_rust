use diesel::prelude::*;
use actix_web::{test, App};
use learn_basic_rust as root;

use root::{
    schema::locations::dsl::*,
    models::location::{Newlocation, Location},
    apis::location_api::{location_api_service, NewLoc}, 
    database::establish_connection,
};


#[actix_web::test]
pub async fn test_get_all_location(){
    let app = test::init_service(
        App::new()
        .configure(location_api_service)
    ).await;

    let req = test::TestRequest::get().uri("/location").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success())
}

#[actix_web::test]
pub async fn test_create_location(){
    let conn = &mut establish_connection();
    let app = test::init_service(
        App::new()
        .configure(location_api_service)
    ).await;
    
    let payload = NewLoc {
        region: "Test".to_string()
    };

    let req = test::TestRequest::post().uri("/location").set_json(&payload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    diesel::delete(locations.filter(region.eq(&payload.region))).execute(conn).unwrap();
}

#[actix_web::test]
pub async fn test_get_location(){
    let conn = &mut establish_connection();

    let app = test::init_service(
        App::new()
        .configure(location_api_service)
    ).await;

    let payload = Newlocation {
        region: &"Test".to_string()
    };

    let req = test::TestRequest::post().uri("/location").set_json(&payload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let location = locations.filter(region.eq(&payload.region)).first::<Location>(conn).unwrap();

    let req = test::TestRequest::get().uri(&format!("/location/{}", &location.id)).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    diesel::delete(locations.filter(region.eq(&payload.region))).execute(conn).unwrap();
}

#[actix_web::test]
pub async fn test_delete_location(){
    let conn = &mut establish_connection();

    let app = test::init_service(
        App::new()
        .configure(location_api_service)
    ).await;

    let payload = Newlocation {
        region: &"Test".to_string()
    };

    let req = test::TestRequest::post().uri("/location").set_json(&payload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let location = locations.filter(region.eq(&payload.region)).first::<Location>(conn).unwrap();

    let req = test::TestRequest::delete().uri(&format!("/location/{}", &location.id)).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}