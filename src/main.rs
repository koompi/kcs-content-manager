mod admins_handler;
mod categories;
mod db_handler;
mod file_handler;
mod file_property;
mod tools;

use actix_web::{App, HttpServer};
use config::Config;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, str::FromStr, sync::Mutex};

lazy_static! {
    static ref CONF_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn get_value_mutex_safe(key: &str) -> String {
    let map_lock = CONF_MAP.lock().unwrap();
    map_lock.get_key_value(key).unwrap().1.to_owned()
}

fn set_init_parameter(conf_location: &str) {
    let settings = Config::builder()
        .add_source(config::File::with_name(conf_location))
        .build()
        .unwrap();
    settings
        .try_deserialize::<HashMap<String, String>>()
        .unwrap()
        .into_iter()
        .for_each(|(key, value)| {
            CONF_MAP.lock().unwrap().insert(key, value);
        })
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let env_args = std::env::args().skip(1).collect::<Vec<String>>();

    match env_args.len() <= 0 {
        true => Err(String::from("Usage: command [CONF_LOCATION]")),
        false => {
            set_init_parameter(&env_args[0]);
            Ok(())
        }
    }
    .unwrap();

    db_handler::migrations::run_init_migration();

    let ip_addr_port = get_value_mutex_safe("IPADDR_PORT");
    // let content_path = get_value_mutex_safe("CONTENTS_ROOT");

    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .supports_credentials(),
            )
            .service(categories::get_sidebar)
            .service(file_handler::upload_api::upload)
            // .service(file_handler::delete_api::delete)
            .service(file_handler::query_api::query_all)
            // .service(file_handler::query_api::query_by_grade)
            // .service(file_handler::query_api::query_by_grade_subject)
            .service(file_handler::query_api::query_by_grade_subject_filename)
            .service(admins_handler::login_api::login)
            // .service(admins_handler::add_admin::add_admin)
            // .service(admins_handler::delete_admin::delete_admin)
            // .service(admins_handler::edit_admin::edit_admin)
            // .service(admins_handler::query_admin::query_admin)
            // .service(file_handler::serve_api::get_file)
    })
    .bind(&ip_addr_port)?;
    println!("Server running at: {}", &ip_addr_port);
    server.run().await
}
