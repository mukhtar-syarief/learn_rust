use crate::models::users::{Users, NewUser};
use diesel::prelude::*;

pub fn create_new_user(conn: &mut PgConnection, username: &String, password: &String) -> Users {
    use crate::schema::users;

    let new_user = NewUser { username, password };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Failed to create user")
}

pub fn user_auth(conn: &mut PgConnection, user_name: &String, pw: &String) -> Users {
    use crate::schema::users::dsl::*;

    let results = users
    .filter(username.eq(user_name))
    .first::<Users>(conn)
    .expect("Error loading posts");
    
    if &results.password != pw {
        panic!("Password is wrong for user {}", user_name)
    }
    else{
        results
    }
}

pub fn get_user_by_username(conn: &mut PgConnection, _username: &String) -> Result<Users, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let user = users.filter(username.eq(_username)).first(conn);
    user
}

pub fn get_all(conn: &mut PgConnection) -> Result<Vec<Users>, diesel::result::Error> {
    use crate::schema::users::dsl::*;

    let all_user = users.load::<Users>(conn);
    all_user
}