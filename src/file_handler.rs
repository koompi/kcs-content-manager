use super::FromStr;
use super::{
    categories::{Grades, Subjects},
    db_handler,
    file_property::{FileRole, FileType}, db_handler::tbl_admins_handler,
    get_value_mutex_safe, Serialize, admins_handler::validate_token
};
use actix_multipart::{Field, Multipart};
use actix_web::{delete, error, post, web, Error, HttpRequest, HttpResponse, get};

#[derive(Debug, Serialize)]
pub struct FileGroup {
    display_name: String,
    filename: String,
    location: String,
    grade: Grades,
    subject: Subjects,
    file_type: FileType,
    thumbnail: Option<Thumbnail>,
}

impl FileGroup {
    pub fn init() -> Self {
        FileGroup {
            display_name: String::new(),
            filename: String::new(),
            location: String::new(),
            grade: Grades::None,
            subject: Subjects::None,
            file_type: FileType::None,
            thumbnail: None,
        }
    }
    pub fn new(
        display_name: String,
        filename: String,
        location: String,
        grade: Grades,
        subject: Subjects,
        file_type: FileType,
        thumbnail: Option<Thumbnail>,
    ) -> Self {
        FileGroup {
            display_name,
            filename,
            location,
            grade,
            subject,
            file_type,
            thumbnail,
        }
    }
    pub fn get_display_name(&self) -> &str {
        self.display_name.as_ref()
    }
    pub fn get_filename(&self) -> &str {
        self.filename.as_ref()
    }
    pub fn get_location(&self) -> &str {
        self.location.as_ref()
    }
    pub fn get_grade(&self) -> &Grades {
        &self.grade
    }
    pub fn get_subject(&self) -> &Subjects {
        &self.subject
    }
    pub fn get_file_type(&self) -> &FileType {
        &self.file_type
    }
    pub fn get_thumbnail(&self) -> Option<&Thumbnail> {
        self.thumbnail.as_ref()
    }

    pub fn set_thumbnail(&mut self, thumbnail: Thumbnail) {
        self.thumbnail = Some(thumbnail);
    }

    fn is_empty(&self) -> bool {
        if self.display_name.is_empty()
            && self.location.is_empty()
            && self.filename.is_empty()
            && self.file_type == FileType::None
        {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Thumbnail {
    thumbnail_name: String,
    thumbnail_location: String,
}
impl Thumbnail {
    pub fn default(filetype: &FileType) -> Option<Self> {
        match filetype {
            FileType::PDF => Some(Thumbnail::new(
                get_value_mutex_safe("PDF_THUMBNAIL_NAME"),
                get_value_mutex_safe("PDF_THUMBNAIL_LOCATION"),
            )),
            FileType::Video => Some(Thumbnail::new(
                get_value_mutex_safe("VIDEO_THUMBNAIL_NAME"),
                get_value_mutex_safe("VIDEO_THUMBNAIL_LOCATION"),
            )),
            FileType::Audio => Some(Thumbnail::new(
                get_value_mutex_safe("AUDIO_THUMBNAIL_NAME"),
                get_value_mutex_safe("AUDIO_THUMBNAIL_LOCATION"),
            )),
            FileType::None => None,
        }
    }
    pub fn new(thumbnail_name: String, thumbnail_location: String) -> Self {
        Thumbnail {
            thumbnail_name,
            thumbnail_location,
        }
    }
    pub fn get_name(&self) -> &str {
        &self.thumbnail_name
    }
    pub fn get_location(&self) -> &str {
        &self.thumbnail_location
    }
}

impl ToOwned for Thumbnail {
    type Owned = Thumbnail;

    fn to_owned(&self) -> Self::Owned {
        Thumbnail::new(
            self.clone().thumbnail_name.to_owned(),
            self.clone().thumbnail_location.to_owned(),
        )
    }

    fn clone_into(&self, target: &mut Self::Owned) {
        *target = self.to_owned();
    }
}

fn extract_url_arg(req: &HttpRequest, arg: &str, err: String) -> Result<String, Error> {
    match req.match_info().get(arg) {
        Some(arg) => Ok(arg.to_owned()),
        None => Err(error::ErrorInternalServerError(err)),
    }
}

pub mod delete_api;
pub mod upload_api;
pub mod query_api;