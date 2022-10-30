use super::{
    delete, error, tbl_admins_handler, validate_token, web, Error, HttpRequest, HttpResponse,
    LoginRole,
};

#[delete("/private/api/admin/delete")]
pub async fn delete_admin(
    req: HttpRequest,
    username: web::Json<String>,
) -> Result<HttpResponse, Error> {
    let (role, claims) = match validate_token(&req) {
        Ok((role, claims)) => Ok((role, claims)),
        Err((code, message)) => match code {
            401 => Err(actix_web::error::ErrorGone(message)),
            _ => Err(actix_web::error::ErrorUnauthorized(message)),
        },
    }?;

    match role {
        LoginRole::Root => Ok(()),
        _ => Err(error::ErrorUnauthorized(String::from("Unauthorised user"))),
    }?;

    match tbl_admins_handler::query_existence_of_admin(claims.get_aud()) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError(String::from(
            "This Root doesn't exists",
        )))
    }?;

    match tbl_admins_handler::query_existence_of_admin(&username) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError(String::from(
            "User doesn't exist",
        ))),
    }?;

    tbl_admins_handler::delete_from_tbl_admins(&username.into_inner());

    Ok(HttpResponse::Ok().finish())
}
