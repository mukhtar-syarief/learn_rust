use crate::models::users::{Users, NewUser};
use diesel::prelude::*;

pub fn create_new_user(conn: &mut PgConnection, username: &String, password: &String) -> Users {
    use crate::schema::users;

    let new_user: NewUser = NewUser { username, password };

    diesel::insert_into(users::table)
        .values(&new_user)
        .get_result(conn)
        .expect("Failed to create user")
}

pub fn user_auth(conn: &mut PgConnection, user_name: &String, pw: &String) -> Users {
    use crate::schema::users::dsl::*;

    let results: Users = users
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

pub fn get_user_by_username(conn: &mut PgConnection, _username: &String) -> Users {
    use crate::schema::users::dsl::*;

    let user: Users = users.filter(username.eq(_username)).first(conn).expect("User not found");
    user
}

pub fn get_all(conn: &mut PgConnection) -> Vec<Users> {
    use crate::schema::users::dsl::*;

    let all_user: Vec<Users> = users.order_by(id).load::<Users>(conn).expect("Kesalahan tidak diketaui");
    all_user
}

pub fn edit_user(conn: &mut PgConnection, user_id: &i32, new_name: &String, new_pw: &String) {
    use crate::schema::users::dsl::*;
    
    diesel::update(users.find(user_id))
                .set((
                    username.eq(new_name), 
                    password.eq(new_pw))
                )
                .execute(conn).unwrap();
}

pub fn delete_user(conn: &mut PgConnection, user_name: &String) {
   use crate::schema::users::dsl::*;
   
   diesel::delete(users.filter(username.eq(user_name))).execute(conn).expect("User gagal dihapus");
}