// use super::rocket;
// use rocket::http::Status;
// use rocket::local::blocking::Client;

// #[test]
// fn hello_world() {
//     let client = Client::tracked(rocket()).expect("valid rocket instance");
//     let response = client.get("/health").dispatch();
//     assert_eq!(response.status(), Status::Ok);
// }