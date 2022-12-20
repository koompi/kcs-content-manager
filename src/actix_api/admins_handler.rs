use super::{
    body, db_handler::tbl_admins_handler, delete, error, extract_url_arg, fmt, get,
    get_value_mutex_safe, http, post, put, web, Deserialize, Error, FromStr, HttpRequest,
    HttpResponse, SearchParameters, Serialize,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct AdminsInfo {
    user_id: Option<String>,
    display_name: Option<String>,
    username: Option<String>,
    password: Option<String>,
    role: Option<LoginRole>,
}

impl AdminsInfo {
    pub fn new(
        user_id: Option<String>,
        display_name: Option<String>,
        username: Option<String>,
        password: Option<String>,
        role: Option<LoginRole>,
    ) -> Self {
        Self {
            user_id,
            display_name,
            username,
            password,
            role,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct LoginModel {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LoginRole {
    Admin,
    Root,
    None,
}

impl FromStr for LoginRole {
    type Err = String;

    fn from_str(input: &str) -> Result<LoginRole, Self::Err> {
        match input {
            "Admin" | "ADMIN" | "រដ្ឋបាល" | "admin" => Ok(LoginRole::Admin),
            "ROOT" | "Root" | "root" => Ok(LoginRole::Root),
            "None" | "NONE" | "none" => Ok(LoginRole::None),
            _ => Err(String::from("Mismatch role: Admin, Root")),
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
    pub fn get_aud(&self) -> &String {
        &self.aud
    }

    fn new(aud: String, role: LoginRole, iat: u64, exp: u64) -> Claims {
        Self {
            aud,
            role,
            iat,
            exp,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    page_count: u32,
    current_page_number: u32,
    data: Vec<AdminsInfo>,
}
impl SearchResponse {
    pub fn new(page_count: u32, current_page_number: u32, data: Vec<AdminsInfo>) -> Self {
        Self {
            page_count,
            current_page_number,
            data,
        }
    }
}

fn validate_password(username: &str, password: &str) -> bool {
    let user_id = tbl_admins_handler::get_user_id_from_username(username);
    let password_hash = tbl_admins_handler::get_password_hash(&user_id);
    bcrypt::verify(password, &password_hash).unwrap()
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

pub fn validate_token(req: &HttpRequest) -> Result<(LoginRole, Claims), (u32, String)> {
    let token = match req.headers().get("AUTHORIZATION") {
        Some(token) => Ok(token.to_str().unwrap().split_whitespace().last().unwrap()),
        None => Err((410, "Token Missing".to_string())),
    }?;

    let claims = match extract_claims_from_token(&token) {
        Ok(claims) => Ok(claims),
        Err((code, message)) => Err((code, message)),
    }?;

    Ok((claims.role.to_owned(), claims))
}

pub mod add_admin;
pub mod delete_admin;
pub mod edit_admin;
pub mod login_api;
pub mod query_admin;
