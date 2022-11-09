use super::{
    error, get, tbl_admins_handler, validate_token, Error, HttpRequest,
    HttpResponse, file_handler
};

#[get("/private/api/admin/query")]
pub async fn query_all_admin(req: HttpRequest) -> Result<HttpResponse, Error> {
    let (_, claims) = match validate_token(&req) {
        Ok((role, claims)) => Ok((role, claims)),
        Err((code, message)) => match code {
            401 => Err(actix_web::error::ErrorGone(message)),
            _ => Err(actix_web::error::ErrorUnauthorized(message)),
        },
    }?;

    match tbl_admins_handler::query_existence_of_admin(claims.get_aud()) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError(String::from(
            "This Root doesn't exists",
        )))
    }?;

    Ok(HttpResponse::Ok().json(
        tbl_admins_handler::query_all_from_tbl_admins()
    ))
}

#[get("/private/api/admin/query/{user_id}")]
pub async fn query_admin_by_id(req: HttpRequest) -> Result<HttpResponse, Error> {
    let (_, claims) = match validate_token(&req) {
        Ok((role, claims)) => Ok((role, claims)),
        Err((code, message)) => match code {
            401 => Err(actix_web::error::ErrorGone(message)),
            _ => Err(actix_web::error::ErrorUnauthorized(message)),
        },
    }?;

    match tbl_admins_handler::query_existence_of_admin(claims.get_aud()) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError(String::from(
            "This Root doesn't exists",
        )))
    }?;

    let user_id = &file_handler::extract_url_arg(
        &req,
        "user_id",
        String::from("Check if user_id URL Arg is valid"),
    )?;

    match tbl_admins_handler::query_existence_of_admin_by_id(user_id) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError(String::from(
            "User doesn't exist",
        ))),
    }?;

    Ok(HttpResponse::Ok().json(
        tbl_admins_handler::query_from_tbl_admins_by_id(&user_id)
    ))
}

#[get("/private/api/admin/search/{search_string}")]
pub async fn search_admin(req: HttpRequest) -> Result<HttpResponse, Error> {
    let (_, claims) = match validate_token(&req) {
        Ok((role, claims)) => Ok((role, claims)),
        Err((code, message)) => match code {
            401 => Err(actix_web::error::ErrorGone(message)),
            _ => Err(actix_web::error::ErrorUnauthorized(message)),
        },
    }?;

    match tbl_admins_handler::query_existence_of_admin(claims.get_aud()) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError(String::from(
            "This Root doesn't exists",
        )))
    }?;

    let search_string = &file_handler::extract_url_arg(
        &req,
        "search_string",
        String::from("Check if search_string URL Arg is valid"),
    )?;

    Ok(HttpResponse::Ok().json(
        tbl_admins_handler::search_from_tbl_admins(&search_string)
    ))
}