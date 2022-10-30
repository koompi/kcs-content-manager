use std::str::FromStr;

use super::{get_value_mutex_safe, params, Connection, admins_handler::{
    AdminsInfo, LoginRole
}};

pub fn insert_into_tbl_admins(display_name: &str, username: &str, passwordhash: &str, role: &str) {
    let database = get_value_mutex_safe("DATABASE");

    Connection::open(&database)
    .unwrap()
    .execute(
        "INSERT INTO tblAdmins VALUES (?1, ?2, ?3, ?4)",
        params![
            display_name,
            username,
            passwordhash,
            role
        ],
    )
    .unwrap();
}

pub fn update_tbl_admins_where(display_name: &str, username: &str, passwordhash: &str, role: &str) {
    let database = get_value_mutex_safe("DATABASE");

    Connection::open(&database)
    .unwrap()
    .execute(
        "UPDATE tblAdmins SET DisplayName=?1, PasswordHash=?3, Role=?4 WHERE UserName=?2",
        params![
            display_name,
            username,
            passwordhash,
            role
        ],
    )
    .unwrap();
}

pub fn delete_from_tbl_admins(username: &str) {
    let database = get_value_mutex_safe("DATABASE");
    Connection::open(&database)
        .unwrap()
        .execute(
            "DELETE FROM tblAdmins WHERE UserName=? ",
            &[username],
        )
        .unwrap();
}

pub fn query_existence_of_admin(username: &str) -> bool {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT UserName FROM tblAdmins WHERE UserName=? LIMIT 1;")
        .unwrap();
    stmt.exists(params![username]).unwrap()
}

pub fn get_password_hash(username: &str) -> String {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT PasswordHash FROM tblAdmins WHERE UserName=? ")
        .unwrap();

    stmt.query_row(params![username], |row| {
        Ok(row.get::<usize, String>(0).unwrap())
    }).unwrap()
}

pub fn get_display_name(username: &str) -> String {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT DisplayName FROM tblAdmins WHERE UserName=? ")
        .unwrap();

    stmt.query_row(params![username], |row| {
        Ok(row.get::<usize, String>(0).unwrap())
    }).unwrap()
}

pub fn get_role(username: &str) -> String {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT Role FROM tblAdmins WHERE UserName=? ")
        .unwrap();

    stmt.query_row(params![username], |row| {
        Ok(row.get::<usize, String>(0).unwrap())
    }).unwrap()
}

pub fn query_all_from_tbl_admins() -> Vec<AdminsInfo> {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT * FROM tblAdmins")
        .unwrap();

    let rows = stmt.query([]);
    let mut admin_lists: Vec<AdminsInfo> = Vec::new();

    if let Ok(mut rows) = rows {
        while let Some(row) = rows.next().unwrap() {
            let display_name: String = row.get(0).unwrap();
            let username: String = row.get(1).unwrap();
            let role_str: String = row.get(3).unwrap();
            let role: LoginRole = LoginRole::from_str(&role_str).unwrap();
            admin_lists.push(
                AdminsInfo::new(Some(display_name), username, None, Some(role))
            )
        }
    }

    admin_lists
}