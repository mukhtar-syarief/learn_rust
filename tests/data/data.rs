
use diesel::prelude::*;
use learn_basic_rust as root;

use root::{
    schema::{
        users,
        locations,
        cars,
    },
    models::{
        users::{NewUser, Users},
        location::{Newlocation, Location},
        cars::{Cars, NewCars},
    }
};


pub fn create_user(conn: &mut PgConnection) -> Users {
    let payload = NewUser {
        username: "test",
        password: "test"
    };

    let user = diesel::insert_into(users::table).values(&payload).get_result::<Users>(conn).unwrap();
    user
}

pub fn delete_user(conn: &mut PgConnection){
    use root::schema::users::dsl::*;
    diesel::delete(users.filter(username.eq("test"))).execute(conn).unwrap();
}

pub fn create_location(conn: &mut PgConnection) -> Location {
    let payload = Newlocation {
        region : &"test".to_string()
    };
    let loc = diesel::insert_into(locations::table).values(&payload).get_result::<Location>(conn).unwrap();
    loc
}

pub fn delete_location(conn: &mut PgConnection) {
    use root::schema::locations::dsl::*;
    diesel::delete(locations.filter(region.eq("test"))).execute(conn).unwrap();
}

pub fn create_car_type(conn: &mut PgConnection) -> Cars {
    let payload = NewCars {
        type_: &"test".to_string()
    };
    let car = diesel::insert_into(cars::table).values(&payload).get_result::<Cars>(conn).unwrap();
    car
}

pub fn delete_car(conn: &mut PgConnection) {
    use root::schema::cars::dsl::*;
    diesel::delete(cars.filter(type_.eq("test"))).execute(conn).unwrap();
}

// pub fn create_invoice(conn: &mut PgConnection) -> ReturnReservation {
//     let loc = create_location(conn);
//     let user = create_user(conn);
//     let payload = NewReturnReservation {
//         full_tank: &true,
//         odometer: &1500,
//         region_id: &loc.id,
//         time: None,
//         user_id: &user.id,
//     };
//     let invoice = diesel::insert_into(return_reservation::table).values(&payload).get_result::<ReturnReservation>(conn).unwrap();
//     invoice
// }

// pub fn delete_invoice(conn: &mut PgConnection, id_user: &i32) {
//     use root::schema::return_reservation::dsl::*;
//     diesel::delete(return_reservation.filter(user_id.eq(id_user))).execute(conn).unwrap();
// }