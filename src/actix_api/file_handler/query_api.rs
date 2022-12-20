use super::{
    db_handler::tbl_contents_handler, error, extract_url_arg, get, web, Error, FileType, FromStr,
    Grades, HttpRequest, HttpResponse, SearchParameters, SearchResponse, Subjects,
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

#[get("/public/api/search")]
pub async fn seatch_contents(
    search_param: web::Query<SearchParameters>,
) -> Result<HttpResponse, Error> {
    let search_string = search_param.get_search_string();
    let name_string = format!("%{}%", search_string);
    let file_type_string = format!(
        "%{}%",
        match FileType::from_str(&search_string) {
            Ok(t) => t,
            Err(_) => FileType::None,
        }
        .to_string()
    );
    let grade_string = format!(
        "%{}%",
        match Grades::from_str(&search_string) {
            Ok(t) => t,
            Err(_) => Grades::None,
        }
        .to_string()
    );
    let subject_string = format!(
        "%{}%",
        match Subjects::from_str(&search_string) {
            Ok(t) => t,
            Err(_) => Subjects::None,
        }
        .to_string()
    );
    let (row_count, db_query_result) = tbl_contents_handler::search_from_tbl_contents(
        &name_string,
        &file_type_string,
        &grade_string,
        &subject_string,
        search_param.get_result_limit(),
        search_param.get_page_number(),
    );
    let page_number = match search_param.page_number {
        Some(t) => t,
        None => 1,
    };
    let mut data_len = row_count / search_param.get_result_limit();
    
    if  (data_len * search_param.get_result_limit()) != row_count  {
        data_len = data_len + 1;
    }

    Ok(HttpResponse::Ok().json(SearchResponse::new(data_len, page_number, db_query_result)))
}

#[get("/public/api/query")]
pub async fn query_all() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(tbl_contents_handler::query_all_from_tbl_contents()))
}
