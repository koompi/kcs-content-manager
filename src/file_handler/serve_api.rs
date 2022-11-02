use actix_files::NamedFile;
use super::{extract_url_arg, error, HttpRequest, Error, get};

#[get("/{file:.*}")]
pub async fn get_file(req: HttpRequest) -> Result<NamedFile, Error> {
    let file_path = match extract_url_arg(&req, "file", String::from("Check if file is available")) {
        Ok(path) => Ok(path),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }?;
    Ok(NamedFile::open(file_path)?)
}