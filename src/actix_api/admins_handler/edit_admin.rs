use super::{
    error, extract_url_arg, put, tbl_admins_handler, validate_token, web, AdminsInfo, Error,
    FromStr, HttpRequest, HttpResponse, LoginRole,
};

#[put("/private/api/admin/edit/{user_id}")]
pub async fn edit_admin(
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

    let mut arg = arg.into_inner();

    if let None = arg.display_name {
        arg.display_name = Some(tbl_admins_handler::get_display_name(&user_id));
    }

    if let None = arg.username {
        arg.username = Some(tbl_admins_handler::get_username(&user_id))
    } else {
        match tbl_admins_handler::query_existence_of_admin(arg.username.as_ref().unwrap()) {
            true => match tbl_admins_handler::get_username(&user_id)
                == arg.username.to_owned().unwrap()
            {
                true => Ok(()),
                false => Err(error::ErrorInternalServerError(String::from(
                    "New Username already exists",
                ))),
            },
            false => Ok(()),
        }?;
    }

    if let None = arg.role {
        arg.role = Some(LoginRole::from_str(&tbl_admins_handler::get_role(&user_id)).unwrap());
    }

    if let None = arg.password {
        arg.password = Some(tbl_admins_handler::get_password_hash(&user_id));
    } else {
        arg.password = Some(bcrypt::hash(arg.password.unwrap(), bcrypt::DEFAULT_COST).unwrap());
    }

    tbl_admins_handler::update_tbl_admins_where(
        &user_id,
        &arg.display_name.to_owned().unwrap(),
        &arg.username.to_owned().unwrap(),
        &arg.password.unwrap(),
        &arg.role.as_ref().to_owned().unwrap().to_string(),
    );

    Ok(HttpResponse::Ok().finish())
}
