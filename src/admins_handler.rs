use std::{str::FromStr, fmt};
use actix_web::{error, post, web, Error, HttpResponse, HttpRequest};
use super::{
    Serialize, Deserialize, get_value_mutex_safe,
    db_handler::tbl_admins_handler
};
use bcrypt::verify;

#[derive(Serialize, Deserialize)]
pub struct LoginModel {
    username: String,
    password: String
}

#[derive(Serialize, Deserialize)]
enum LoginRole {
    Admin,
    Root,
    None,
}

impl FromStr for LoginRole {
    type Err = String;

    fn from_str(input: &str) -> Result<LoginRole, Self::Err> {
        match input {
            "Admin" => Ok(LoginRole::Admin),
            "ADMIN" => Ok(LoginRole::Admin),
            "admin" => Ok(LoginRole::Admin),
            "ROOT" => Ok(LoginRole::Root),
            "Root" => Ok(LoginRole::Root),
            "root" => Ok(LoginRole::Root),
            "None" => Ok(LoginRole::None),
            "NONE" => Ok(LoginRole::None),
            "none" => Ok(LoginRole::None),
            _ => Err(String::from(
                "Mismatch role: Admin, Root"
            )),
        }
    }
}

impl fmt::Display for LoginRole {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoginRole::Admin => write!(f, "Admin"),
            LoginRole::Root => write!(f, "Root"),
            LoginRole::None => write!(f, "None"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    role: LoginRole,
    iat: u64,
    exp: u64,
}

impl Claims {
    fn new(aud: String, role: LoginRole, iat: u64, exp: u64) -> Claims {
        Self {
            aud,
            role,
            iat,
            exp,
        }
    }
}

fn validate_password(username: &str, password: &str) -> bool {
    let password_hash = tbl_admins_handler::get_password_hash(username);
    verify(password, &password_hash).unwrap()
}

fn extract_claims_from_token(token: &str) -> Result<Claims, (u32, String)> {
    let mut validation = jsonwebtoken::Validation::new(jsonwebtoken::Algorithm::HS256);
    validation.set_required_spec_claims(&["aud", "role", "iat", "exp"]);
    validation.validate_exp = true;
    let token_message = match jsonwebtoken::decode::<Claims>(
        &token,
        &jsonwebtoken::DecodingKey::from_secret(get_value_mutex_safe("DECRYPT_KEY").as_ref()),
        &validation,
    ) {
        Ok(token) => Ok(token),
        Err(_err) => Err((410, String::from("Token expired or incorrect"))),
    }?;

    Ok(token_message.claims)
}

fn validate_token(req: &HttpRequest) -> Result<LoginRole, (u32, String)> {
    let token = match req.headers().get("AUTHORIZATION") {
        Some(token) => Ok(token.to_str().unwrap().split_whitespace().last().unwrap()),
        None => Err((410, "Token Missing".to_string())),
    }?;

    let claims = match extract_claims_from_token(&token) {
        Ok(claims) => Ok(claims),
        Err((code, message)) => Err((code, message)),
    }?;

    Ok(claims.role)
}

pub mod login_api;
pub mod add_admin;