use utoipa::OpenApi;

use crate::{
    apis::{
        user as users_api,
        cars_api,
        location_api,
        reservation_api,
        return_reservation_api as invoice_api,

    },
    models::{
        users as user_model,
        cars as cars_model,
        location as location_model,
        reservation as reservation_model,
        return_reservation as invoice_model
    }
};


#[derive(OpenApi)]
#[openapi(
    paths(
        users_api::get_all_user,
        users_api::create_user,
        users_api::get_user,
        users_api::update_user,
        users_api::delete_this_user,
        cars_api::get_all_cars_type,
        cars_api::create_car_type,
        cars_api::delete_car_type,
        location_api::get_locations,
        location_api::get_loc,
        location_api::create_location,
        location_api::delete_loc,
        reservation_api::get_reservations,
        reservation_api::get_reservation,
        reservation_api::create_user_reservation,
        reservation_api::edit_this_reservation,
        reservation_api::delete_this_reservation,
        invoice_api::get_return_reservations,
        invoice_api::get_return_reservation,
        invoice_api::create_return_reservation,
        invoice_api::edit_return_reservation,
        invoice_api::delete_return_reservation
    ),
    components(
        schemas(
            users_api::UserAuth,
            users_api::MessageResponse,
            cars_api::CarPayload,
            reservation_api::ReservationPayload,
            invoice_api::InvoicePayload,
            user_model::Users,
            cars_model::Cars,
            location_model::Location,
            reservation_model::Reservation,
            invoice_model::ReturnReservation
        )
    ),
    tags(
        (
            name = "users_api", 
            description = "This API use to manage user. Inside this is Api to get all user or only one user, create new user, update data of user, also to delete."
        ),
        (
            name = "cars_api", 
            description = "This API is use to get all type of car and register new type. Also in this api can update if you insert wrong type car or you choose to delete type from the list."
        ),
        (
            name = "location_api", 
            description = "API to manage location. You want to get location data or register new location can use this API."
        ),
        (
            name = "reservation_api",
            description = "Reservation or invoice API to display detail reservation of user. You can manage invoice arround CRUD in this API."
        ),
        (
            name = "invoice_api",
            description = "This API use to give invoice to user was reservation."
        )
    ),
)]
pub struct ApiDoc;