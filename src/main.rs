use clap::Parser;
use sea_orm::Database;
mod cli;
mod entities;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let db_connection_string = format!(
        "mysql://{}:{}@{}/{}",
        cli.db_username, cli.db_password, cli.db_host, cli.db_name,
    );
    let db = match Database::connect(&db_connection_string).await {
        Ok(db) => {
            hw_msg::infoln!("Connected to database.");
            db
        }
        Err(err) => {
            hw_msg::errorln!("Failed to connect to database. [{}]", err);
            quit::with_code(exitcode::UNAVAILABLE);
        }
    };

    let exit_code = match cli.command {
        _ => todo!(),
    };
}
