use super::{
    error, extract_url_arg, post, db_handler, Error, Field, FileGroup, FileRole, FileType, FromStr,
    Grades, HttpRequest, HttpResponse, Multipart, Subjects, Thumbnail, get_value_mutex_safe
};
use crate::tools;
use futures_util::stream::StreamExt as _;
use std::{
    fs,
    io::{prelude::*, BufWriter},
    path,
};

#[post("/private/api/upload/{grade}/{subject}/{type}")]
pub async fn upload(req: HttpRequest, mut payload: Multipart) -> Result<HttpResponse, Error> {
    let mut file_group: FileGroup = FileGroup::init();
    let subject = match Subjects::from_str(&extract_url_arg(
        &req,
        "subject",
        String::from("Check if Subject URL Arg is valid"),
    )?) {
        Ok(subjects) => Ok(subjects),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }?;
    // let subject_str = subject.to_string();
    let grade = match Grades::from_str(&extract_url_arg(
        &req,
        "grade",
        String::from("Check if Grade URL Arg is valid"),
    )?) {
        Ok(subjects) => Ok(subjects),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }?;
    // let grade_str = grade.to_string();

    while let Some(item) = payload.next().await {
        let mut field: Field = item?;
        let file_disposition = field.content_disposition();
        let display_name = file_disposition.get_filename().unwrap().to_string();
        let file_type = match FileType::from_str(&extract_url_arg(
            &req,
            "type",
            String::from("Check if FileType URL Arg is valid"),
        )?) {
            Ok(file_type) => Ok(file_type),
            Err(err) => Err(error::ErrorInternalServerError(err)),
        }?;
        let file_extension = file_disposition
            .get_filename()
            .unwrap()
            .split(".")
            .last()
            .unwrap();

        let filename = tools::generate_random(
            100,
            Some(file_disposition.get_filename().unwrap().to_string()),
        ) + "."
            + file_extension;

        let file_role = match FileRole::from_str(file_disposition.get_name().unwrap()) {
            Ok(role) => Ok(role),
            Err(err) => Err(error::ErrorInternalServerError(err)),
        }?;
        let root_path = get_value_mutex_safe("CONTENTS_ROOT");
        let location = format!("{}/{}/{}", root_path, &grade, &subject);
        let fullpath = format!("{}/{}", location, filename);

        let file_obj = tools::continue_file(fullpath.as_ref());

        while let Some(chunk) = field.next().await {
            let mut write_buffer = BufWriter::new(&file_obj);
            write_buffer.write(&chunk.unwrap())?;
        }

        match file_role {
            FileRole::ContentFile => {
                let default_thumbnail = match file_group.get_thumbnail() {
                    Some(thumbnail) => Some(thumbnail.to_owned()),
                    None => Thumbnail::default(&file_type),
                };
                file_group = FileGroup::new(
                    display_name,
                    filename,
                    location,
                    grade,
                    subject,
                    file_type,
                    default_thumbnail,
                )
            }
            FileRole::ThumbnailFile => file_group.set_thumbnail(Thumbnail::new(filename, location)),
        };
    }

    if file_group.is_empty() {
        let thumbnail = file_group.get_thumbnail().unwrap();
        let fullpath_thumbnail = thumbnail.get_location().to_owned() + "/" + thumbnail.get_name();

        // Ignore Error because if it doesn't exist, its fine
        fs::remove_file(path::Path::new(&fullpath_thumbnail)).unwrap_or(());

        Err(error::ErrorBadRequest(
            "Thumbnail uploaded without Main File. File is needed.",
        ))
    } else {
        let thumbnail = file_group.get_thumbnail().unwrap();
        db_handler::tbl_contents_handler::insert_into_contents_table(
            file_group.get_display_name(),
            file_group.get_filename(),
            file_group.get_location(),
            file_group.get_file_type().to_string().as_ref(),
            file_group.get_grade().to_string().as_ref(),
            file_group.get_subject().to_string().as_ref(),
            thumbnail.get_name(),
            thumbnail.get_location(),
        );
        Ok(())
    }?;

    Ok(HttpResponse::Ok().finish())
}
