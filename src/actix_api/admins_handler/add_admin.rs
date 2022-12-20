use super::{
    error, post, tbl_admins_handler, validate_token, web, AdminsInfo, Error, HttpRequest,
    HttpResponse, LoginRole,
};

#[post("/private/api/admin/add")]
pub async fn add_admin(
    req: HttpRequest,
    arg: web::Json<AdminsInfo>,
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
        ))),
    }?;

    match tbl_admins_handler::query_existence_of_admin(&arg.username.to_owned().unwrap()) {
        true => Err(error::ErrorInternalServerError(String::from(
            "User already exists",
        ))),
        false => Ok(()),
    }?;

    if let None = arg.display_name {
        Err(error::ErrorBadRequest(String::from("Missing Display Name")))
    } else if let None = arg.role {
        Err(error::ErrorBadRequest(String::from("Missing Role")))
    } else if let None = arg.password {
        Err(error::ErrorBadRequest(String::from("Missing Password")))
    } else if let None = arg.username {
        Err(error::ErrorBadRequest(String::from("Missing Username")))
    } else {
        Ok(())
    }?;

    tbl_admins_handler::insert_into_tbl_admins(
        &uuid::Uuid::new_v4().hyphenated().to_string(),
        &arg.display_name.to_owned().unwrap(),
        &arg.username.to_owned().unwrap(),
        &bcrypt::hash::<String>(arg.password.to_owned().unwrap(), bcrypt::DEFAULT_COST).unwrap(),
        &arg.role.as_ref().to_owned().unwrap().to_string(),
    );

    Ok(HttpResponse::Ok().finish())
}
