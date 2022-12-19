use std::str::FromStr;

use super::{
    admins_handler::{AdminsInfo, LoginRole},
    get_value_mutex_safe, params, Connection,
};

pub fn insert_into_tbl_admins(
    user_id: &str,
    display_name: &str,
    username: &str,
    passwordhash: &str,
    role: &str,
) {
    let database = get_value_mutex_safe("DATABASE");

    Connection::open(&database)
        .unwrap()
        .execute(
            "INSERT INTO tblAdmins VALUES (?1, ?2, ?3, ?4, ?5)",
            params![user_id, display_name, username, passwordhash, role],
        )
        .unwrap();
}

pub fn update_tbl_admins_where(
    user_id: &str,
    display_name: &str,
    username: &str,
    passwordhash: &str,
    role: &str,
) {
    let database = get_value_mutex_safe("DATABASE");

    Connection::open(&database)
    .unwrap()
    .execute(
"UPDATE tblAdmins 
SET DisplayName=?2, UserName=?3, PasswordHash=?4, Role=?5 
WHERE UserID=?1",
        params![
            user_id,
            display_name,
            username,
            passwordhash,
            role
        ],
    )
    .unwrap();
}

pub fn delete_from_tbl_admins(user_id: &str) {
    let database = get_value_mutex_safe("DATABASE");
    Connection::open(&database)
        .unwrap()
        .execute("DELETE FROM tblAdmins WHERE UserID=? ", &[user_id])
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

pub fn query_existence_of_admin_by_id(user_id: &str) -> bool {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT UserName FROM tblAdmins WHERE UserID=? LIMIT 1;")
        .unwrap();
    stmt.exists(params![user_id]).unwrap()
}

pub fn get_user_id_from_username(username: &str) -> String {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT UserID FROM tblAdmins WHERE UserName=? ")
        .unwrap();

    stmt.query_row(params![username], |row| {
        Ok(row.get::<usize, String>(0).unwrap())
    })
    .unwrap()
}

pub fn get_password_hash(user_id: &str) -> String {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT PasswordHash FROM tblAdmins WHERE UserID=? ")
        .unwrap();

    stmt.query_row(params![user_id], |row| {
        Ok(row.get::<usize, String>(0).unwrap())
    })
    .unwrap()
}

pub fn get_display_name(user_id: &str) -> String {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT DisplayName FROM tblAdmins WHERE UserID=? ")
        .unwrap();

    stmt.query_row(params![user_id], |row| {
        Ok(row.get::<usize, String>(0).unwrap())
    })
    .unwrap()
}

pub fn get_username(user_id: &str) -> String {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT UserName FROM tblAdmins WHERE UserID=? ")
        .unwrap();

    stmt.query_row(params![user_id], |row| {
        Ok(row.get::<usize, String>(0).unwrap())
    })
    .unwrap()
}

pub fn get_role(user_id: &str) -> String {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT Role FROM tblAdmins WHERE UserID=? ")
        .unwrap();

    stmt.query_row(params![user_id], |row| {
        Ok(row.get::<usize, String>(0).unwrap())
    })
    .unwrap()
}

pub fn query_all_from_tbl_admins() -> Vec<AdminsInfo> {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT UserID,DisplayName,UserName,Role FROM tblAdmins")
        .unwrap();

    let rows = stmt.query([]);
    let mut admin_lists: Vec<AdminsInfo> = Vec::new();

    if let Ok(mut rows) = rows {
        while let Some(row) = rows.next().unwrap() {
            let user_id: String = row.get(0).unwrap();
            let display_name: String = row.get(1).unwrap();
            let username: String = row.get(2).unwrap();
            let role_str: String = row.get(3).unwrap();
            let role: LoginRole = LoginRole::from_str(&role_str).unwrap();
            admin_lists.push(AdminsInfo::new(
                Some(user_id),
                Some(display_name),
                Some(username),
                None,
                Some(role),
            ))
        }
    }

    admin_lists
}

pub fn query_from_tbl_admins_by_id(user_id: &str) -> AdminsInfo {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare("SELECT DisplayName,UserName,Role FROM tblAdmins WHERE UserID=?")
        .unwrap();

    stmt.query_row(params![user_id], |row| {
        let display_name: String = row.get(0).unwrap();
        let username: String = row.get(1).unwrap();
        let role_str: String = row.get(2).unwrap();
        let role: LoginRole = LoginRole::from_str(&role_str).unwrap();
        Ok(AdminsInfo::new(
            Some(user_id.to_string()),
            Some(display_name),
            Some(username),
            None,
            Some(role),
        ))
    })
    .unwrap()
}

pub fn search_from_tbl_admins(
    username_string: &str,
    disp_name_string: &str,
    role_string: &str,
    result_limit: &u32,
    page_number: Option<u32>,
) -> (u32, Vec<AdminsInfo>) {
    let database = get_value_mutex_safe("DATABASE");
    let connection = Connection::open(&database).unwrap();

    let mut stmt = connection
        .prepare(
"SELECT COUNT(*) 
FROM tblAdmins
WHERE Username LIKE ?1 OR 
DisplayName LIKE ?2 OR 
Role LIKE ?3",
        )
        .unwrap();

    let row_count = stmt
        .query_row(
            params![username_string, disp_name_string, role_string,],
            |row| Ok(row.get::<usize, u32>(0).unwrap()),
        )
        .unwrap();

    let rows = match page_number {
        Some(page_number) => {
            stmt = connection
                .prepare(
"SELECT UserID,DisplayName,UserName,Role 
FROM tblAdmins
WHERE Username LIKE ?1 OR 
DisplayName LIKE ?2 OR 
Role LIKE ?3 
LIMIT ? 
OFFSET ?",
                )
                .unwrap();
            stmt.query(params![
                username_string,
                disp_name_string,
                role_string,
                result_limit,
                (page_number - 1) * result_limit
            ])
        }
        None => {
            stmt = connection
                .prepare(
"SELECT UserID,DisplayName,UserName,Role 
FROM tblAdmins
WHERE Username LIKE ?1 OR 
DisplayName LIKE ?2 OR 
Role LIKE ?3 
LIMIT ?",
                )
                .unwrap();
            stmt.query(params![
                username_string,
                disp_name_string,
                role_string,
                result_limit,
            ])
        }
    };

    let mut admin_lists: Vec<AdminsInfo> = Vec::new();

    if let Ok(mut rows) = rows {
        while let Some(row) = rows.next().unwrap() {
            let user_id: String = row.get(0).unwrap();
            let display_name: String = row.get(1).unwrap();
            let username: String = row.get(2).unwrap();
            let role_str: String = row.get(3).unwrap();
            let role: LoginRole = LoginRole::from_str(&role_str).unwrap();
            admin_lists.push(AdminsInfo::new(
                Some(user_id),
                Some(display_name),
                Some(username),
                None,
                Some(role),
            ))
        }
    }

    (row_count, admin_lists)
}
