use rusqlite::Rows;

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

pub fn delete_from_tbl_contents(filename: &str, grade: &str, subject: &str) {
    let database = get_value_mutex_safe("DATABASE");
    Connection::open(&database)
        .unwrap()
        .execute(
            "DELETE FROM tblContents WHERE FileName=? AND Grade=? AND Subject=?",
            &[filename, grade, subject],
        )
        .unwrap();
}

pub fn query_existence_of_file(filename: &str, grade: &str, subject: &str) -> bool {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare(
            "SELECT FileName FROM tblContents WHERE Filename=? AND Grade=? AND Subject=? LIMIT 1",
        )
        .unwrap();
    stmt.exists(params![filename, grade, subject]).unwrap()
}

pub fn query_from_tbl_contents_with_grade_subject_file_id(grade: &str, subject: &str, file_id: &str) -> FileGroup {
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
        Ok(FileGroup::new(file_id, display_name, filename, location, grade, subject, file_type, Some(thumbnail), grade_kh, subject_kh))
    }).unwrap()

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

pub fn query_all_from_tbl_contents() -> Vec<FileGroup> {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT * FROM tblContents")
        .unwrap();

    let rows = stmt.query([]);

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
            subject_kh
        ));
    }

    file_lists
}
