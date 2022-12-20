use super::{
    Grades, Subjects, FromStr,
    db_handler, http, fs, Path,
    FileRole, FileType, db_handler::tbl_admins_handler, Thumbnail,
    get_value_mutex_safe, Serialize, admins_handler::validate_token, FileGroup,
    delete, error, post, Error, HttpRequest, HttpResponse, get, web, SearchParameters
};
use actix_multipart::{Field, Multipart};

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    page_count: u32,
    current_page_number: u32,
    data: Vec<FileGroup>
}
impl SearchResponse {
    pub fn new(page_count: u32, current_page_number: u32, data: Vec<FileGroup>) -> Self {
        Self {
            page_count, current_page_number, data
        }
    }
}

pub fn extract_url_arg(req: &HttpRequest, arg: &str, err: String) -> Result<String, Error> {
    match req.match_info().get(arg) {
        Some(arg) => Ok(arg.to_owned()),
        None => Err(error::ErrorInternalServerError(err)),
    }
}

pub mod delete_api;
pub mod upload_api;
pub mod query_api;
pub mod serve_api;