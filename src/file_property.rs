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
