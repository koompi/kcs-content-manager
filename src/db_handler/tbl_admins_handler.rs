use super::{get_value_mutex_safe, params, Connection};

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