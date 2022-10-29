use super::{get_value_mutex_safe, Grades, Subjects, fs, Path, Connection};

pub fn run_init_migration() {
    let database = get_value_mutex_safe("DATABASE");

    if !std::path::Path::new(&database).exists() {
        Connection::open(&database).unwrap().execute_batch(
"BEGIN;
CREATE TABLE tblContents (
    DisplayName VARCHAR(255), FileName NVARCHAR(100) NOT NULL PRIMARY KEY UNIQUE, Location VARCHAR(255), FileType CHARACTER(20), 
    Grade INT, Subject NVARCHAR(100), ThumbnailName NVARCHAR(100), ThumbnailLocation VARCHAR(255)
);
CREATE TABLE tblAdmins (DisplayName NVARCHAR(100), UserName NVARCHAR(100) NOT NULL PRIMARY KEY UNIQUE, 
    PasswordHash NVARCHAR(100)
);
COMMIT;").unwrap();
    }

    let root_path = get_value_mutex_safe("CONTENTS_ROOT");
    let path_obj_to_root = Path::new(&root_path);
    if !path_obj_to_root.exists() {
        fs::create_dir(path_obj_to_root).unwrap_or(());
    }

    Grades::iterator().for_each(|each_grade| {
        let path_str_to_grade = format!("{}/{}", root_path, each_grade.to_string());
        let path_obj_to_grade = Path::new(&path_str_to_grade);
        if !path_obj_to_grade.exists() {
            fs::create_dir(path_obj_to_grade).unwrap_or(());
        }

        Subjects::iterator().for_each(|each_subject| {
            let path_str_to_subject = format!(
                "{}/{}/{}",
                root_path,
                each_grade.to_string(),
                each_subject.to_string()
            );
            let path_obj_to_subject = Path::new(&path_str_to_subject);
            if !path_obj_to_subject.exists() {
                fs::create_dir(path_obj_to_subject).unwrap_or(());
            }
        });
    });
}
