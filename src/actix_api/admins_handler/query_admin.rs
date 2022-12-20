use super::{
    error, extract_url_arg, get, tbl_admins_handler, validate_token, web, Error, FromStr,
    HttpRequest, HttpResponse, LoginRole, SearchParameters, SearchResponse,
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
        ))),
    }?;

    Ok(HttpResponse::Ok().json(tbl_admins_handler::query_all_from_tbl_admins()))
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
        ))),
    }?;

    let user_id = extract_url_arg(
        &req,
        "user_id",
        String::from("Check if user_id URL Arg is valid"),
    )?;

    match tbl_admins_handler::query_existence_of_admin_by_id(user_id.as_ref()) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError(String::from(
            "User doesn't exist",
        ))),
    }?;

    Ok(HttpResponse::Ok().json(tbl_admins_handler::query_from_tbl_admins_by_id(&user_id)))
}

#[get("/private/api/admin/search")]
pub async fn search_admin(
    req: HttpRequest,
    search_parameter: web::Query<SearchParameters>,
) -> Result<HttpResponse, Error> {
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
        ))),
    }?;

    let name_string = format!("%{}%", search_parameter.get_search_string());
    let role_string = format!(
        "%{}%",
        match LoginRole::from_str(&name_string) {
            Ok(t) => t,
            Err(_) => LoginRole::None,
        }
        .to_string()
    );

    let (row_count, db_query_result) = tbl_admins_handler::search_from_tbl_admins(
        &name_string,
        &name_string,
        &role_string,
        search_parameter.get_result_limit(),
        search_parameter.get_page_number(),
    );

    let page_number = match search_parameter.get_page_number() {
        Some(t) => t,
        None => 1,
    };

    let mut data_len = row_count / search_parameter.get_result_limit();

    if (data_len * search_parameter.get_result_limit()) != row_count {
        data_len = data_len + 1;
    }

    Ok(HttpResponse::Ok().json(SearchResponse::new(data_len, page_number, db_query_result)))
}
