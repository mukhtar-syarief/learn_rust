use actix_web::{test, App};
use diesel::prelude::*;

use learn_basic_rust as root;
use crate::data::data::{
    create_car_type,
    create_location,
    create_user,
    delete_car,
    delete_location, 
    delete_user,
};

use root::{
    schema::{
        reservations::dsl::*,
    },
    models::{
        reservation::Reservation,
    },
    apis::{reservation_api::{reservation_api_services, ReservationPayload}},
    database::establish_connection,
};

#[test]
pub async fn test_get_user_all_reservation(){
    let conn = &mut establish_connection();
    let app = test::init_service(
        App::new()
        .configure(reservation_api_services)
    ).await;

    let user = create_user(conn);

    let req = test::TestRequest::get().uri(&format!("/reservation/{}", &user.username)).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
    delete_user(conn);
}


#[test]
pub async fn test_create_reservation() {
    let conn = &mut establish_connection();

    let app = test::init_service(
        App::new()
        .configure(reservation_api_services)
    ).await;

    let user = create_user(conn);
    let loc = create_location(conn);
    let car = create_car_type(conn);

    let payload = ReservationPayload {
        region_id: loc.id.clone(),
        vehicle_type_id: car.id.clone(),
        pickup_date: 1670832104000,
        return_date: 1670832104000

    };

    let req = test::TestRequest::post().uri(&format!("/reservation/{}", &user.username)).set_json(&payload).to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", resp.status());
    assert!(resp.status().is_success());

    diesel::delete(reservations.filter(user_id.eq(&user.id))).execute(conn).unwrap();
    delete_user(conn);
    delete_location(conn);
    delete_car(conn);

}


#[test]
pub async fn test_get_user_reservation() {
    let conn = &mut establish_connection();

    let app = test::init_service(
        App::new()
        .configure(reservation_api_services)
    ).await;

    let user = create_user(conn);
    let loc = create_location(conn);
    let car = create_car_type(conn);

    let payload = ReservationPayload {
        region_id: loc.id.clone(),
        vehicle_type_id: car.id.clone(),
        pickup_date: 1670832104000,
        return_date: 1670832104000

    };

    let req = test::TestRequest::post().uri(&format!("/reservation/{}", &user.username)).set_json(&payload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let reservation = reservations.filter(user_id.eq(&user.id)).first::<Reservation>(conn).unwrap();

    let path = &format!("/reservation/{}/{}", &user.username, &reservation.id);
    println!("{}", path);

    let req = test::TestRequest::get().uri(&format!("/reservation/{}/{}", &user.username, &reservation.id)).to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", resp.status());
    assert!(resp.status().is_success());

    diesel::delete(reservations.filter(user_id.eq(&user.id))).execute(conn).unwrap();
    delete_user(conn);
    delete_location(conn);
    delete_car(conn);
}


#[test]
pub async fn test_edit_user_reservation() {
    let conn = &mut establish_connection();

    let app = test::init_service(
        App::new()
        .configure(reservation_api_services)
    ).await;

    let user = create_user(conn);
    let loc = create_location(conn);
    let car = create_car_type(conn);

    let payload = ReservationPayload {
        region_id: loc.id.clone(),
        vehicle_type_id: car.id.clone(),
        pickup_date: 1670832104000,
        return_date: 1670832104000

    };

    let req = test::TestRequest::post().uri(&format!("/reservation/{}", &user.username)).set_json(&payload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let edit_payload = ReservationPayload {
        region_id: loc.id.clone(),
        vehicle_type_id: car.id.clone(),
        pickup_date: 1670832104000,
        return_date: 1671093821000
    };

    let reservation = reservations.filter(user_id.eq(&user.id)).first::<Reservation>(conn).unwrap();

    let req = test::TestRequest::put().uri(&format!("/reservation/{}/{}", &user.username, &reservation.id)).set_json(&edit_payload).to_request();
    let resp = test::call_service(&app, req).await;
    println!("{}", resp.status());
    assert!(resp.status().is_success());

    diesel::delete(reservations.filter(user_id.eq(&user.id))).execute(conn).unwrap();
    delete_user(conn);
    delete_location(conn);
    delete_car(conn);
}


#[test]
pub async fn test_delete_user_reservation() {
    let conn = &mut establish_connection();

    let app = test::init_service(
        App::new()
        .configure(reservation_api_services)
    ).await;

    let user = create_user(conn);
    let loc = create_location(conn);
    let car = create_car_type(conn);

    let payload = ReservationPayload {
        region_id: loc.id.clone(),
        vehicle_type_id: car.id.clone(),
        pickup_date: 1670832104000,
        return_date: 1670832104000

    };

    let req = test::TestRequest::post().uri(&format!("/reservation/{}", &user.username)).set_json(&payload).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let reservation = reservations.filter(user_id.eq(&user.id)).first::<Reservation>(conn).unwrap();
    
    let req = test::TestRequest::delete().uri(&format!("/reservation/{}/{}", &user.username, &reservation.id)).to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    delete_user(conn);
    delete_location(conn);
    delete_car(conn);
}