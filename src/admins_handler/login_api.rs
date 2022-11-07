use super::{
    error, get_value_mutex_safe, post, tbl_admins_handler, validate_password, web, Claims, Error,
    FromStr, HttpResponse, LoginModel, LoginRole,
};

#[post("/public/api/login")]
pub async fn login(login_args: web::Json<LoginModel>) -> Result<HttpResponse, Error> {
    let username = &login_args.username.to_owned();
    let password = &login_args.password.to_owned();

    match tbl_admins_handler::query_existence_of_admin(username) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError(String::from(
            "Incorrect Username or Password",
        ))),
    }?;

    match validate_password(username, password) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError(String::from(
            "Incorrect Username or Password",
        ))),
    }?;

    let timestamp = jsonwebtoken::get_current_timestamp();
    let user_id = tbl_admins_handler::get_user_id_from_username(username);
    let current_role = tbl_admins_handler::get_role(user_id.as_ref());

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &Claims::new(
            username.to_owned(),
            LoginRole::from_str(&current_role).unwrap(),
            timestamp,
            timestamp + get_value_mutex_safe("TOKEN_EXPIRATION_SEC").parse::<u64>().unwrap(),
        ),
        &jsonwebtoken::EncodingKey::from_secret(get_value_mutex_safe("DECRYPT_KEY").as_ref()),
    )
    .unwrap();
    Ok(HttpResponse::Ok().json(token))
}
