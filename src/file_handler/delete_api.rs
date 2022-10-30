use super::{
    error, extract_url_arg, db_handler::tbl_contents_handler, delete, web, Error, 
    FromStr, Grades, HttpRequest, HttpResponse, Subjects, tbl_admins_handler, validate_token
};
#[delete("/private/api/delete/{grade}/{subject}")]
pub async fn delete(req: HttpRequest, filename: web::Json<String>) -> Result<HttpResponse, Error> {
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

    let grade = match Grades::from_str(&extract_url_arg(
        &req,
        "grade",
        String::from("Check if Grade URL Arg is valid"),
    )?) {
        Ok(subjects) => Ok(subjects),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }?.to_string();

    let subject = match Subjects::from_str(&extract_url_arg(
        &req,
        "subject",
        String::from("Check if Subject URL Arg is valid"),
    )?) {
        Ok(subjects) => Ok(subjects),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }?.to_string();

    let filename = filename.into_inner();

    match tbl_contents_handler::query_existence_of_file(
        &filename,
        &grade,
        &subject,
    ) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError("File doesn't exists"))
    }?;

    tbl_contents_handler::delete_from_tbl_contents(&filename, &grade, &subject);



    Ok(HttpResponse::Ok().finish())
}
