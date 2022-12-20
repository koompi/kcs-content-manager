mod actix_api;

use std::{collections::HashMap, sync::Mutex};

lazy_static::lazy_static! {
    static ref CONF_MAP: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}

fn get_value_mutex_safe(key: &str) -> String {
    let map_lock = CONF_MAP.lock().unwrap();
    map_lock.get_key_value(key).unwrap().1.to_owned()
}

fn set_init_parameter(conf_location: &str) {
    let settings = config::Config::builder()
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

    actix_api::db_handler::migrations::run_init_migration();

    let ip_addr_port = get_value_mutex_safe("IPADDR_PORT");

    let server = actix_web::HttpServer::new(move || {
        actix_web::App::new()
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_header()
                    .allow_any_method()
                    .allow_any_origin()
                    .supports_credentials(),
            )
            .service(actix_api::categories::get_sidebar)
            .service(actix_api::file_handler::upload_api::upload)
            .service(actix_api::file_handler::delete_api::delete)
            .service(actix_api::file_handler::delete_api::delete_by_id)
            .service(actix_api::file_handler::query_api::query_all)
            .service(actix_api::file_handler::query_api::query_by_grade)
            .service(actix_api::file_handler::query_api::query_by_grade_subject)
            .service(actix_api::file_handler::query_api::query_by_grade_subject_filename)
            .service(actix_api::file_handler::query_api::seatch_contents)
            .service(actix_api::admins_handler::login_api::login)
            .service(actix_api::admins_handler::add_admin::add_admin)
            .service(actix_api::admins_handler::delete_admin::delete_admin)
            .service(actix_api::admins_handler::edit_admin::edit_admin)
            .service(actix_api::admins_handler::query_admin::query_all_admin)
            .service(actix_api::admins_handler::query_admin::query_admin_by_id)
            .service(actix_api::admins_handler::query_admin::search_admin)
            .service(actix_api::file_handler::serve_api::get_file)
    })
    .bind(&ip_addr_port)?;
    println!("Server running at: {}", &ip_addr_port);
    server.run().await
}
