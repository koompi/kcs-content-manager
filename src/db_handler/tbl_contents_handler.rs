use rusqlite::{Rows, Error};

use super::{get_value_mutex_safe, params, Connection, Grades, Subjects};
use crate::{
    file_handler::{FileGroup, Thumbnail},
    file_property::FileType,
};
use std::str::FromStr;

pub fn insert_into_contents_table(
    file_id: &str,
    display_name: &str,
    filename: &str,
    location: &str,
    file_type: &str,
    grade: &str,
    subject: &str,
    thumbnail_name: &str,
    thumbnail_location: &str,
) {
    let database = get_value_mutex_safe("DATABASE");

    Connection::open(&database)
        .unwrap()
        .execute(
            "INSERT INTO tblContents VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                file_id,
                display_name,
                filename,
                location,
                file_type,
                grade,
                subject,
                thumbnail_name,
                thumbnail_location
            ],
        )
        .unwrap();
}

pub fn delete_from_tbl_contents(file_id: &str, grade: &str, subject: &str) {
    let database = get_value_mutex_safe("DATABASE");
    Connection::open(&database)
        .unwrap()
        .execute(
            "DELETE FROM tblContents WHERE FileID=? AND Grade=? AND Subject=?",
            &[file_id, grade, subject],
        )
        .unwrap();
}

pub fn delete_from_tbl_contents_by_id(file_id: &str) {
    let database = get_value_mutex_safe("DATABASE");
    Connection::open(&database)
        .unwrap()
        .execute("DELETE FROM tblContents WHERE FileID=?", &[file_id])
        .unwrap();
}

pub fn query_existence_of_file(file_id: &str, grade: &str, subject: &str) -> bool {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT FileID FROM tblContents WHERE FileID=? AND Grade=? AND Subject=? LIMIT 1")
        .unwrap();
    stmt.exists(params![file_id, grade, subject]).unwrap()
}

pub fn query_file_thumbnail_location_by_id(file_id: &str) -> (String, String) {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare(
            "SELECT FileName,Location,ThumbnailName,ThumbnailLocation FROM tblContents WHERE FileID=?",
        )
        .unwrap();
    stmt.query_row([file_id], |row| {
        let file_name: String = row.get(0).unwrap();
        let file_location: String = row.get(1).unwrap();
        let thumbnail_name: String = row.get(2).unwrap();
        let thumbnail_location: String = row.get(3).unwrap();
        let file_full_path = file_location + "/" + &file_name;
        let thumb_full_path = thumbnail_location + "/" + &thumbnail_name;
        Ok((file_full_path, thumb_full_path))
    })
    .unwrap()
}

pub fn query_file_thumbnail_location(
    file_id: &str,
    grade: &str,
    subject: &str,
) -> (String, String) {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare(
"SELECT FileName,Location,ThumbnailName,ThumbnailLocation FROM tblContents WHERE FileID=? AND Grade=? AND Subject=?",
        )
        .unwrap();
    stmt.query_row([file_id, grade, subject], |row| {
        let file_name: String = row.get(0).unwrap();
        let file_location: String = row.get(1).unwrap();
        let thumbnail_name: String = row.get(2).unwrap();
        let thumbnail_location: String = row.get(3).unwrap();
        let file_full_path = file_location + "/" + &file_name;
        let thumb_full_path = thumbnail_location + "/" + &thumbnail_name;
        Ok((file_full_path, thumb_full_path))
    })
    .unwrap()
}

pub fn query_existence_of_file_id(file_id: &str) -> bool {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT FileID FROM tblContents WHERE FileID=? LIMIT 1")
        .unwrap();
    stmt.exists(params![file_id]).unwrap()
}

pub fn query_from_tbl_contents_with_grade_subject_file_id(
    grade: &str,
    subject: &str,
    file_id: &str,
) -> FileGroup {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT * FROM tblContents WHERE Grade=? AND Subject=? AND FileID=?")
        .unwrap();

    stmt.query_row([grade, subject, file_id], |row| {
        let file_id: String = row.get(0).unwrap();
        let display_name: String = row.get(1).unwrap();
        let filename: String = row.get(2).unwrap();
        let location: String = row.get(3).unwrap();

        let file_type_str: String = row.get(4).unwrap();
        let file_type: FileType = FileType::from_str(&file_type_str).unwrap();

        let grade_str: String = row.get(5).unwrap();
        let grade: Grades = Grades::from_str(&grade_str).unwrap();

        let subject_str: String = row.get(6).unwrap();
        let subject = Subjects::from_str(&subject_str).unwrap();

        let thumbnail_name: String = row.get(7).unwrap();
        let thumbnail_location: String = row.get(8).unwrap();
        let thumbnail: Thumbnail = Thumbnail::new(thumbnail_name, thumbnail_location);

        let grade_kh = Grades::get_kh(grade);
        let subject_kh = Subjects::get_kh(subject);
        Ok(FileGroup::new(
            file_id,
            display_name,
            filename,
            location,
            grade,
            subject,
            file_type,
            Some(thumbnail),
            grade_kh,
            subject_kh,
        ))
    })
    .unwrap()
}

pub fn query_from_tbl_contents_with_grade_subject(grade: &str, subject: &str) -> Vec<FileGroup> {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT * FROM tblContents WHERE Grade=? AND Subject=?")
        .unwrap();

    let rows = stmt.query([grade, subject]);

    match rows {
        Ok(mut rows) => filter_rows_for_filegroup(&mut rows),
        Err(_) => Vec::new(),
    }
}

pub fn query_from_tbl_contents_with_grade(grade: &str) -> Vec<FileGroup> {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT * FROM tblContents WHERE Grade=? ")
        .unwrap();

    let rows = stmt.query([grade]);

    match rows {
        Ok(mut rows) => filter_rows_for_filegroup(&mut rows),
        Err(_) => Vec::new(),
    }
}

pub fn query_displayname_from_tbl_contents(filename: &str) -> Result<String, Error>  {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT DisplayName FROM tblContents WHERE FileName=?")
        .unwrap();

    stmt.query_row(params![filename], |row| {
        Ok(row.get::<usize, String>(0).unwrap())
    })
}

pub fn query_all_from_tbl_contents() -> Vec<FileGroup> {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection.prepare("SELECT * FROM tblContents").unwrap();

    let rows = stmt.query([]);

    match rows {
        Ok(mut rows) => filter_rows_for_filegroup(&mut rows),
        Err(_) => Vec::new(),
    }
}

pub fn search_from_tbl_contents(search_string: &str) -> Vec<FileGroup> {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare(
            "SELECT * FROM tblContents 
WHERE DisplayName LIKE '%?1%' OR FileType LIKE '%?2%' OR Grade LIKE '%?3%' OR Subject LIKE '%?4%'",
        )
        .unwrap();

    let rows = stmt.query(params![
        search_string,
        search_string,
        search_string,
        search_string
    ]);

    match rows {
        Ok(mut rows) => filter_rows_for_filegroup(&mut rows),
        Err(_) => Vec::new(),
    }
}

fn filter_rows_for_filegroup(rows: &mut Rows) -> Vec<FileGroup> {
    let mut file_lists: Vec<FileGroup> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        let file_id: String = row.get(0).unwrap();
        let display_name: String = row.get(1).unwrap();
        let filename: String = row.get(2).unwrap();
        let location: String = row.get(3).unwrap();

        let file_type_str: String = row.get(4).unwrap();
        let file_type: FileType = FileType::from_str(&file_type_str).unwrap();

        let grade_str: String = row.get(5).unwrap();
        let grade: Grades = Grades::from_str(&grade_str).unwrap();

        let subject_str: String = row.get(6).unwrap();
        let subject = Subjects::from_str(&subject_str).unwrap();

        let thumbnail_name: String = row.get(7).unwrap();
        let thumbnail_location: String = row.get(8).unwrap();
        let thumbnail: Thumbnail = Thumbnail::new(thumbnail_name, thumbnail_location);

        let grade_kh = Grades::get_kh(grade);
        let subject_kh = Subjects::get_kh(subject);

        file_lists.push(FileGroup::new(
            file_id,
            display_name,
            filename,
            location,
            grade,
            subject,
            file_type,
            Some(thumbnail),
            grade_kh,
            subject_kh,
        ));
    }

    file_lists
}
