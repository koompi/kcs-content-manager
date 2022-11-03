use super::{
    db_handler::tbl_contents_handler, error, extract_url_arg, get, Error, FromStr, Grades,
    HttpRequest, HttpResponse, Subjects,
};

#[get("/public/api/query/{grade}/{subject}/{file_id}")]
pub async fn query_by_grade_subject_filename(req: HttpRequest) -> Result<HttpResponse, Error> {
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

    let file_id = &extract_url_arg(
        &req,
        "file_id",
        String::from("Check if file_id URL Arg is valid"),
    )?;

    Ok(HttpResponse::Ok().json(
        tbl_contents_handler::query_from_tbl_contents_with_grade_subject_file_id(
            &grade, &subject, &file_id,
        ),
    ))
}

#[get("/public/api/query/{grade}/{subject}")]
pub async fn query_by_grade_subject(req: HttpRequest) -> Result<HttpResponse, Error> {
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

    Ok(HttpResponse::Ok()
        .json(tbl_contents_handler::query_from_tbl_contents_with_grade_subject(&grade, &subject)))
}

#[get("/public/api/query/{grade}")]
pub async fn query_by_grade(req: HttpRequest) -> Result<HttpResponse, Error> {
    let grade = match Grades::from_str(&extract_url_arg(
        &req,
        "grade",
        String::from("Check if Grade URL Arg is valid"),
    )?) {
        Ok(subjects) => Ok(subjects),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }?
    .to_string();

    Ok(
        HttpResponse::Ok().json(tbl_contents_handler::query_from_tbl_contents_with_grade(
            &grade,
        )),
    )
}

#[get("/public/api/query")]
pub async fn query_all() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(tbl_contents_handler::query_all_from_tbl_contents()))
}
