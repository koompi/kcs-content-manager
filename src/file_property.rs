use super::{fmt, FromStr, Serialize};

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
            "PDF" => Ok(FileType::PDF),
            "pdf" => Ok(FileType::PDF),
            "Pdf" => Ok(FileType::PDF),
            "Video" => Ok(FileType::Video),
            "VIDEO" => Ok(FileType::Video),
            "video" => Ok(FileType::Video),
            "Audio" => Ok(FileType::Audio),
            "AUDIO" => Ok(FileType::Audio),
            "audio" => Ok(FileType::Audio),
            "None" => Ok(FileType::None),
            "NONE" => Ok(FileType::None),
            "none" => Ok(FileType::None),
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
