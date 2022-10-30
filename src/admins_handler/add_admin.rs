use super::{
    error, get_value_mutex_safe, post, tbl_admins_handler, validate_token, web, Claims, Error,
    FromStr, HttpRequest, HttpResponse, LoginModel, LoginRole,
};

#[post("/private/api/admins/add")]
pub async fn add_admins(req: HttpRequest, arg: web::Json<LoginModel>) -> Result<HttpResponse, Error> {
    let role = match validate_token(&req) {
        Ok(role) => Ok(role),
        Err((code, message)) => match code {
            401 => Err(actix_web::error::ErrorGone(message)),
            _ => Err(actix_web::error::ErrorUnauthorized(message)),
        },
    }?;

    match role {
        LoginRole::Root => Ok(()),
        _ => Err(actix_web::error::ErrorUnauthorized(String::from("Unauthorised user")))
    }?;

    Ok(HttpResponse::Ok().finish())
}
