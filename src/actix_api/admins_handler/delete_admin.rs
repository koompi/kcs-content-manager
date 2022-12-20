use super::{
    delete, error, extract_url_arg, tbl_admins_handler, validate_token, Error, HttpRequest,
    HttpResponse, LoginRole,
};

#[delete("/private/api/admin/delete/{user_id}")]
pub async fn delete_admin(req: HttpRequest) -> Result<HttpResponse, Error> {
    let (role, claims) = match validate_token(&req) {
        Ok((role, claims)) => Ok((role, claims)),
        Err((code, message)) => match code {
            401 => Err(actix_web::error::ErrorGone(message)),
            _ => Err(actix_web::error::ErrorUnauthorized(message)),
        },
    }?;

    let user_id = &extract_url_arg(
        &req,
        "user_id",
        String::from("Check if user_id URL Arg is valid"),
    )?;

    match role {
        LoginRole::Root => Ok(()),
        _ => Err(error::ErrorUnauthorized(String::from("Unauthorised user"))),
    }?;

    match tbl_admins_handler::query_existence_of_admin(claims.get_aud()) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError(String::from(
            "This Root doesn't exists",
        ))),
    }?;

    match tbl_admins_handler::query_existence_of_admin_by_id(user_id) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError(String::from(
            "User doesn't exist",
        ))),
    }?;

    tbl_admins_handler::delete_from_tbl_admins(&user_id);

    Ok(HttpResponse::Ok().finish())
}
