use super::{
    admins_handler::validate_token, db_handler, db_handler::tbl_admins_handler, delete, error,
    extract_url_arg, fs, get, get_value_mutex_safe, http, post, web, Error, FileGroup, FileRole,
    FileType, FromStr, Grades, HttpRequest, HttpResponse, Path, SearchParameters, Serialize,
    Subjects, Thumbnail,
};
use actix_multipart::{Field, Multipart};

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    page_count: u32,
    current_page_number: u32,
    data: Vec<FileGroup>,
}
impl SearchResponse {
    pub fn new(page_count: u32, current_page_number: u32, data: Vec<FileGroup>) -> Self {
        Self {
            page_count,
            current_page_number,
            data,
        }
    }
}

pub mod delete_api;
pub mod query_api;
pub mod serve_api;
pub mod upload_api;
