use super::get_value_mutex_safe;
use actix_web::{body, delete, error, get, http, post, put, web, Error, HttpRequest, HttpResponse};
use categories::{Grades, Subjects};
use serde::{Deserialize, Serialize};
use std::{fmt, fs, path::Path, str::FromStr};

#[derive(Deserialize)]
pub struct SearchParameters {
    search_string: String,
    result_limit: u32,
    page_number: Option<u32>,
}

impl SearchParameters {
    pub fn get_search_string(&self) -> &str {
        &self.search_string
    }
    pub fn get_result_limit(&self) -> &u32 {
        &self.result_limit
    }
    pub fn get_page_number(&self) -> Option<u32> {
        self.page_number
    }
}

#[derive(Deserialize)]
pub struct QueryPaginationParameters {
    result_limit: u32,
    page_number: Option<u32>,
}

impl QueryPaginationParameters {
    pub fn get_result_limit(&self) -> &u32 {
        &self.result_limit
    }
    pub fn get_page_number(&self) -> Option<u32> {
        self.page_number
    }
}

#[derive(Debug, PartialEq, Serialize)]
pub enum FileType {
    PDF,
    Video,
    Audio,
    None,
}

impl FromStr for FileType {
    type Err = String;

    fn from_str(input: &str) -> Result<FileType, Self::Err> {
        match input {
            "PDF" | "pdf" | "Pdf" => Ok(FileType::PDF),
            "Video" | "VIDEO" | "វីដេអូ" | "video" => Ok(FileType::Video),
            "Audio" | "AUDIO" | "សម្លេង" | "សំឡេង" | "audio" => {
                Ok(FileType::Audio)
            }
            "None" | "NONE" | "none" => Ok(FileType::None),
            _ => Err(String::from("Mismatch Type: PDF, Video, Audio")),
        }
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::PDF => write!(f, "PDF"),
            FileType::Video => write!(f, "Video"),
            FileType::Audio => write!(f, "Audio"),
            FileType::None => write!(f, "None"),
        }
    }
}

#[derive(Debug)]
pub enum FileRole {
    ContentFile,
    ThumbnailFile,
}

impl FromStr for FileRole {
    type Err = String;

    fn from_str(input: &str) -> Result<FileRole, Self::Err> {
        match input {
            "ContentFile" => Ok(FileRole::ContentFile),
            "contentfile" => Ok(FileRole::ContentFile),
            "CONTENTFILE" => Ok(FileRole::ContentFile),
            "File" => Ok(FileRole::ContentFile),
            "file" => Ok(FileRole::ContentFile),
            "FILE" => Ok(FileRole::ContentFile),
            "ThumbnailFile" => Ok(FileRole::ThumbnailFile),
            "THUMBNAILFILE" => Ok(FileRole::ThumbnailFile),
            "thumbnailfile" => Ok(FileRole::ThumbnailFile),
            "thumbnail" => Ok(FileRole::ThumbnailFile),
            "Thumbnail" => Ok(FileRole::ThumbnailFile),
            "THUMBNAIL" => Ok(FileRole::ThumbnailFile),
            _ => Err(String::from("Mismatch type: File, Thumbnail")),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct FileGroup {
    file_id: String,
    display_name: String,
    filename: String,
    location: String,
    grade: Grades,
    subject: Subjects,
    file_type: FileType,
    thumbnail: Option<Thumbnail>,
    grade_kh: String,
    subject_kh: String,
}

impl FileGroup {
    pub fn init() -> Self {
        FileGroup {
            file_id: String::new(),
            display_name: String::new(),
            filename: String::new(),
            location: String::new(),
            grade: Grades::None,
            subject: Subjects::None,
            file_type: FileType::None,
            thumbnail: None,
            grade_kh: String::new(),
            subject_kh: String::new(),
        }
    }
    pub fn new(
        file_id: String,
        display_name: String,
        filename: String,
        location: String,
        grade: Grades,
        subject: Subjects,
        file_type: FileType,
        thumbnail: Option<Thumbnail>,
        grade_kh: String,
        subject_kh: String,
    ) -> Self {
        FileGroup {
            file_id,
            display_name,
            filename,
            location,
            grade,
            subject,
            file_type,
            thumbnail,
            grade_kh,
            subject_kh,
        }
    }
    pub fn get_file_id(&self) -> &str {
        self.file_id.as_ref()
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
        let clone_self = self.clone();
        Thumbnail::new(
            clone_self.thumbnail_name.to_owned(),
            clone_self.thumbnail_location.to_owned(),
        )
    }

    fn clone_into(&self, target: &mut Self::Owned) {
        *target = self.to_owned();
    }
}

pub fn extract_url_arg(req: &HttpRequest, arg: &str, err: String) -> Result<String, Error> {
    match req.match_info().get(arg) {
        Some(arg) => Ok(arg.to_owned()),
        None => Err(error::ErrorInternalServerError(err)),
    }
}

pub mod admins_handler;
pub mod categories;
pub mod db_handler;
pub mod file_handler;
