use std::str::FromStr;

use super::{
    put, error, tbl_admins_handler, validate_token, web, Error, HttpRequest, HttpResponse,
    LoginRole, AdminsInfo
};

#[put("/private/api/admin/edit")]
pub async fn edit_admin(req: HttpRequest, arg: web::Json<AdminsInfo>) -> Result<HttpResponse, Error> {
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

    match tbl_admins_handler::query_existence_of_admin(&arg.username) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError(String::from(
            "User doesn't exist",
        ))),
    }?;


    let mut arg = arg.into_inner();

    if let None = arg.display_name  {
        arg.display_name = Some(tbl_admins_handler::get_display_name(&arg.username));
    }
    if let None = arg.role {
        arg.role = Some(LoginRole::from_str(&tbl_admins_handler::get_role(&arg.username)).unwrap());
    }
    if let None = arg.password {
        arg.password = Some(tbl_admins_handler::get_password_hash(&arg.username));
    }

    tbl_admins_handler::update_tbl_admins_where(
        &arg.display_name.to_owned().unwrap(),
        &arg.username,
        &arg.password.unwrap(),
        &arg.role.as_ref().to_owned().unwrap().to_string(),
    );

    Ok(HttpResponse::Ok().finish())
}