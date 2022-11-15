use super::{
    db_handler, error, extract_url_arg, get_value_mutex_safe, post, tbl_admins_handler,
    validate_token, Error, Field, FileGroup, FileRole, FileType, FromStr, Grades, HttpRequest,
    HttpResponse, Multipart, Subjects, Thumbnail,
};
use crate::tools;
use futures_util::stream::StreamExt as _;
use std::{
    fs,
    io::{prelude::*, BufWriter},
    path,
    time::SystemTime,
};

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

#[post("/private/api/upload/{grade}/{subject}/{type}")]
pub async fn upload(req: HttpRequest, mut payload: Multipart) -> Result<HttpResponse, Error> {
    let (_, claims) = match validate_token(&req) {
        Ok((role, claims)) => Ok((role, claims)),
        Err((code, message)) => match code {
            401 => Err(actix_web::error::ErrorGone(message)),
            _ => Err(actix_web::error::ErrorUnauthorized(message)),
        },
    }?;

    match tbl_admins_handler::query_existence_of_admin(claims.get_aud()) {
        true => Ok(()),
        false => Err(error::ErrorInternalServerError(String::from(
            "This Root doesn't exists",
        ))),
    }?;

    let mut file_group: FileGroup = FileGroup::init();
    let subject = match Subjects::from_str(&extract_url_arg(
        &req,
        "subject",
        String::from("Check if Subject URL Arg is valid"),
    )?) {
        Ok(subject) => Ok(subject),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }?;
    let grade = match Grades::from_str(&extract_url_arg(
        &req,
        "grade",
        String::from("Check if Grade URL Arg is valid"),
    )?) {
        Ok(grade) => Ok(grade),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }?;

    while let Some(item) = payload.next().await {
        let mut field: Field = item?;
        let file_disposition = field.content_disposition();
        let mut full_filename_split = file_disposition.get_filename().unwrap().split(".");
        let display_name = full_filename_split.next().unwrap().to_string();
        let file_type = match FileType::from_str(&extract_url_arg(
            &req,
            "type",
            String::from("Check if FileType URL Arg is valid"),
        )?) {
            Ok(file_type) => Ok(file_type),
            Err(err) => Err(error::ErrorInternalServerError(err)),
        }?;
        let file_extension = full_filename_split.last().unwrap();

        let filename = uuid::Uuid::from_u128(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis(),
        )
        .hyphenated()
        .to_string()
            + base64::encode(&display_name)
                .replace(
                    &['/', ' ', '\\', '&', '|', ':', ';', '$', '#', '~', '='],
                    "-",
                )
                .as_str();
        let filename = truncate(&filename, 240).to_string() + "." + file_extension;

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
                let file_id = uuid::Uuid::new_v4().hyphenated().to_string();
                let grade_kh = Grades::get_kh(grade);
                let subject_kh = Subjects::get_kh(subject);
                file_group = FileGroup::new(
                    file_id,
                    display_name,
                    filename,
                    location,
                    grade,
                    subject,
                    file_type,
                    default_thumbnail,
                    grade_kh,
                    subject_kh,
                );
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
            file_group.get_file_id(),
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
