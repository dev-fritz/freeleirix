use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Algorithm, Validation, EncodingKey, DecodingKey, TokenData, errors::Error as JwtError};
use chrono::{Duration, Utc};

#[derive(Debug, Deserialize, Serialize)]
pub struct Login {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    exp: usize,
    user_id: i32,
    email: String,
}

pub fn encode_password(password: &str) -> String {
    hash(password, DEFAULT_COST).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap()
}

pub fn generate_token(user_id: i32, email: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(30))
        .expect("valid timestamp")
        .timestamp() as usize;
    
    let my_claims = Claims { 
        exp: expiration, 
        user_id, 
        email: email.to_owned() 
    };
    let token = encode(&Header::default(), &my_claims, &EncodingKey::from_secret("mysecretkey".as_ref())).unwrap();
    token
}

pub fn _validate_token_format(token: &str) -> bool {
    token.split('.').count() == 3
}

pub fn validate_token(token: &str) -> Result<TokenData<Claims>, JwtError> {
    decode::<Claims>(token, &DecodingKey::from_secret("mysecretkey".as_ref()), &Validation::new(Algorithm::HS256))
}