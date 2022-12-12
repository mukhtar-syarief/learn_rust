use actix_web::{test, App};
use diesel::prelude::*;

use learn_basic_rust as root;

use root::{
    schema::return_reservation::dsl::*,
    models::{
        return_reservation::{
            ReturnReservation
        },
    }, apis::return_reservation_api::{return_reservation_api_service, InvoicePayload}, 
    database::establish_connection

};

use crate::data::data::{
    create_user, 
    create_location, 
    create_car_type,
    delete_car,
    delete_location,
    delete_user,
};


#[test]
pub async fn test_get_all_invoice(){
    let conn = &mut establish_connection();

    let app = test::init_service(
        App::new()
        .configure(return_reservation_api_service)
    ).await;

    let user = create_user(conn);

    let req = test::TestRequest::get().uri(&format!("/invoice/{}", &user.username)).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    delete_user(conn);
}

#[test]
pub async fn test_create_invoice(){
    let conn = &mut establish_connection();

    let app = test::init_service(
        App::new()
        .configure(return_reservation_api_service)
    ).await;

    let user = create_user(conn);
    let loc = create_location(conn);
    create_car_type(conn);

    let payload = InvoicePayload {
        user_id: user.id,
        region_id: loc.id,
        odometer: 150,
        full_tank: true,
        time: 1670832104000
    };

    let req = test::TestRequest::post().uri(&format!("/invoice/{}", &user.username)).set_json(&payload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    diesel::delete(return_reservation.filter(user_id.eq(&user.id))).execute(conn).unwrap();
    delete_car(conn);
    delete_location(conn);
    delete_user(conn);
}

#[test]
pub async fn test_get_user_invoice(){
    let conn = &mut establish_connection();

    let app = test::init_service(
        App::new()
        .configure(return_reservation_api_service)
    ).await;

    let user = create_user(conn);
    let loc = create_location(conn);
    create_car_type(conn);

    let payload = InvoicePayload {
        user_id: user.id,
        region_id: loc.id,
        odometer: 150,
        full_tank: true,
        time: 1670832104000
    };

    let req = test::TestRequest::post().uri(&format!("/invoice/{}", &user.username)).set_json(&payload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let invoice = return_reservation.filter(user_id.eq(&user.id)).first::<ReturnReservation>(conn).unwrap();

    let req = test::TestRequest::get().uri(&format!("/invoice/{}/{}", &user.username, &invoice.id)).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    diesel::delete(return_reservation.filter(user_id.eq(&user.id))).execute(conn).unwrap();
    delete_car(conn);
    delete_location(conn);
    delete_user(conn);
}

#[test]
pub async fn test_edit_invoice(){
    let conn = &mut establish_connection();

    let app = test::init_service(
        App::new()
        .configure(return_reservation_api_service)
    ).await;

    let user = create_user(conn);
    let loc = create_location(conn);
    create_car_type(conn);

    let payload = InvoicePayload {
        user_id: user.id,
        region_id: loc.id,
        odometer: 150,
        full_tank: true,
        time: 1670832104000
    };

    let req = test::TestRequest::post().uri(&format!("/invoice/{}", &user.username)).set_json(&payload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let invoice = return_reservation.filter(user_id.eq(&user.id)).first::<ReturnReservation>(conn).unwrap();

    let edit_payload = InvoicePayload {
        user_id: user.id,
        region_id: loc.id,
        odometer: 200,
        full_tank: false,
        time: 1670832104000
    };

    let req = test::TestRequest::put().uri(&format!("/invoice/{}/{}", &user.username, &invoice.id)).set_json(&edit_payload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    diesel::delete(return_reservation.filter(user_id.eq(&user.id))).execute(conn).unwrap();
    delete_car(conn);
    delete_location(conn);
    delete_user(conn);
}

#[test]
pub async fn test_delete_invoice(){
    let conn = &mut establish_connection();

    let app = test::init_service(
        App::new()
        .configure(return_reservation_api_service)
    ).await;

    let user = create_user(conn);
    let loc = create_location(conn);
    create_car_type(conn);

    let payload = InvoicePayload {
        user_id: user.id,
        region_id: loc.id,
        odometer: 150,
        full_tank: true,
        time: 1670832104000
    };

    let req = test::TestRequest::post().uri(&format!("/invoice/{}", &user.username)).set_json(&payload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let invoice = return_reservation.filter(user_id.eq(&user.id)).first::<ReturnReservation>(conn).unwrap();

    let req = test::TestRequest::delete().uri(&format!("/invoice/{}/{}", &user.username, &invoice.id)).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    delete_car(conn);
    delete_location(conn);
    delete_user(conn);
}