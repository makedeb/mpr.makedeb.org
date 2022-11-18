use rocket::{Config, config::{Ident, LogLevel}, data::Limits};
use std::net::IpAddr;
use colored::Colorize;
use crate::routes;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use include_dir::include_dir;
use tera::Tera;

pub async fn start(db: DatabaseConnection, address: String, port: u16, log_level: LogLevel) -> i32 {
    let ip_address: IpAddr = match address.parse() {
        Ok(ip_addr) => ip_addr,
        Err(err) => {
            hw_msg::errorln!("Failed to parse '{}' as an IP address. [{}]", address.green(), err);
            return exitcode::USAGE;
        }
    };

    let config = Config {
        address: ip_address,
        port: port,
        log_level: log_level,
        ..Default::default()
    };

    let server_routes = rocket::routes![
        routes::media::media_assets,
        routes::misc::home
    ];

    // A function to get all the files from an `include_dir::Dir` struct.
    // We define this as a function and not a closure because closures can't recursively call themselves.
    fn get_files(dir: include_dir::Dir) -> Vec<include_dir::File> {
        let mut files = vec![];

        for file in dir.files() {
            files.push(file.to_owned());
        }

        for directory in dir.dirs() {
            files.append(&mut get_files(directory.to_owned()));
        }

        files
    }
    let templates = get_files(
        include_dir!("$CARGO_MANIFEST_DIR/templates")
    );

    hw_msg::infoln!("Preparing HTTP server...");
    let ignition = match rocket::custom(&config)
        .manage(db)
        .mount("/", server_routes)
        .ignite().await {
        Ok(rocket) => rocket,
        Err(err) => {
            hw_msg::errorln!("Failed to prepare HTTP server. [{}]", err);
            return exitcode::UNAVAILABLE;
        }
    };

    hw_msg::infoln!("Launching HTTP server...");
    if let Err(err) = ignition.launch().await {
        hw_msg::errorln!("Failed to launch HTTP server. [{}]", err);
        return exitcode::UNAVAILABLE;
    }

    exitcode::OK
}