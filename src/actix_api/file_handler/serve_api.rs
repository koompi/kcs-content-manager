use super::{
    db_handler, error, extract_url_arg, get,
    http::header::{Charset, ContentDisposition, DispositionParam, DispositionType, ExtendedValue},
    Error, HttpRequest,
};
use actix_files::NamedFile;

#[get("/{file:.*}")]
pub async fn get_file(req: HttpRequest) -> Result<NamedFile, Error> {
    let file_path = match extract_url_arg(&req, "file", String::from("Check if file is available"))
    {
        Ok(path) => Ok(path),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }?;
    let file_name = file_path.split("/").last().unwrap();
    match db_handler::tbl_contents_handler::query_displayname_from_tbl_contents(file_name) {
        Ok(display_name) => {
            let name_file = NamedFile::open(&file_path)?;
            let file_extension = file_name.split(".").last().unwrap();
            let true_name = display_name + "." + file_extension;
            let cd1 = ContentDisposition {
                disposition: DispositionType::Attachment,
                parameters: vec![DispositionParam::FilenameExt(ExtendedValue {
                    charset: Charset::Ext(String::from("UTF-8")),
                    language_tag: None,
                    value: true_name.as_bytes().to_vec(),
                })],
            };
            Ok(name_file.set_content_disposition(cd1))
        }
        Err(_) => Ok(NamedFile::open(&file_path)?),
    }
}
