use std::{str::FromStr, fmt};
use actix_web::{error, post, web, Error, HttpResponse};
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

pub mod login_api;
