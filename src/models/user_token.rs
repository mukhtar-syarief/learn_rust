use serde::{Serialize, Deserialize};
// use jsonwebtoken as jwt;

#[derive(Serialize, Deserialize)]
pub struct Token {
    exp: String,
    token: String,
}


#[derive(Serialize, Deserialize)]
pub struct UserToken {
    id: i32,
    username: i32
}



// const SECRET_KEY: &str = "secret"; 


// impl UserToken {
//     pub fn generate_token(&self) -> Token {

//     }
// }