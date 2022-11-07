use super::{
    db_handler::tbl_contents_handler, delete, error, extract_url_arg, tbl_admins_handler,
    validate_token, web, Error, FromStr, Grades, HttpRequest, HttpResponse, Subjects,
};

#[delete("/private/api/delete/{file_id}")]
pub async fn delete_by_id(req: HttpRequest) -> Result<HttpResponse, Error> {
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

    let file_id = &extract_url_arg(
        &req,
        "file_id",
        String::from("Check if file_id URL Arg is valid"),
    )?;

    match tbl_contents_handler::query_existence_of_file_id(&file_id) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError("File doesn't exists")),
    }?;

    let (file, thumb) = tbl_contents_handler::query_file_thumbnail_location_by_id(&file_id);
    std::fs::remove_file(file).unwrap_or(());
    std::fs::remove_file(thumb).unwrap_or(());
    tbl_contents_handler::delete_from_tbl_contents_by_id(&file_id);

    Ok(HttpResponse::Ok().finish())
}

#[delete("/private/api/delete/{grade}/{subject}")]
pub async fn delete(req: HttpRequest, file_id: web::Json<String>) -> Result<HttpResponse, Error> {
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

    let grade = match Grades::from_str(&extract_url_arg(
        &req,
        "grade",
        String::from("Check if Grade URL Arg is valid"),
    )?) {
        Ok(subjects) => Ok(subjects),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }?
    .to_string();

    let subject = match Subjects::from_str(&extract_url_arg(
        &req,
        "subject",
        String::from("Check if Subject URL Arg is valid"),
    )?) {
        Ok(subjects) => Ok(subjects),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }?
    .to_string();

    let file_id = file_id.into_inner();

    match tbl_contents_handler::query_existence_of_file(&file_id, &grade, &subject) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError("File doesn't exists")),
    }?;

    let (file, thumb) =
        tbl_contents_handler::query_file_thumbnail_location(&file_id, &grade, &subject);
    std::fs::remove_file(file).unwrap_or(());
    std::fs::remove_file(thumb).unwrap_or(());

    tbl_contents_handler::delete_from_tbl_contents(&file_id, &grade, &subject);

    Ok(HttpResponse::Ok().finish())
}
