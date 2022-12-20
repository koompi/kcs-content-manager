use super::{fs, get_value_mutex_safe, Connection, Grades, Path, Subjects};

pub fn run_init_migration() {
    let database = get_value_mutex_safe("DATABASE");
    let init_username: String = get_value_mutex_safe("ROOT_INITIAL_USERNAME");
    let init_password: String = bcrypt::hash(
        get_value_mutex_safe("ROOT_INITIAL_PASSWORD"),
        bcrypt::DEFAULT_COST,
    )
    .unwrap();

    if !std::path::Path::new(&database).exists() {
        Connection::open(&database)
            .unwrap()
            .execute_batch(
                format!(
                    "BEGIN;
CREATE TABLE tblContents(
    FileID NVARCHAR(100) NOT NULL PRIMARY KEY UNIQUE, 
    DisplayName VARCHAR(255), 
    FileName NVARCHAR(100) NOT NULL, 
    Location VARCHAR(255) NOT NULL, 
    FileType CHARACTER(20) NOT NULL, 
    Grade NVARCHAR(100) NOT NULL, 
    Subject NVARCHAR(100) NOT NULL, 
    ThumbnailName NVARCHAR(100) NOT NULL, 
    ThumbnailLocation VARCHAR(255) NOT NULL
);
CREATE TABLE tblAdmins(
    UserID NVARCHAR(100) NOT NULL PRIMARY KEY UNIQUE, 
    DisplayName NVARCHAR(100), 
    UserName NVARCHAR(100) NOT NULL UNIQUE, 
    PasswordHash NVARCHAR(100) NOT NULL, 
    Role NVARCHAR(100) NOT NULL
);
INSERT INTO tblAdmins('UserID', DisplayName, UserName, PasswordHash, Role) 
    VALUES('{}', 'Root', '{}', '{}', 'Root');
COMMIT;",
                    &uuid::Uuid::new_v4().hyphenated().to_string(),
                    init_username,
                    init_password
                )
                .as_str(),
            )
            .unwrap();
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
