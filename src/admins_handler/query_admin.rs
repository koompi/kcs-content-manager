use super::{
    error, get, tbl_admins_handler, validate_token, Error, HttpRequest,
    HttpResponse,
};

#[get("/private/api/admin/query")]
pub async fn query_admin(req: HttpRequest) -> Result<HttpResponse, Error> {
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