use clap::Parser;
use sea_orm::Database;
mod cli;
mod entities;
mod http;
mod routes;
use cli::{Cli, Commands};
use migration::{Migrator, MigratorTrait};
use dotenv::dotenv;
mod util;

#[tokio::main]
async fn main() {
    // Read environment variables.
    dotenv().ok();

    // Get the CLI.
    let cli = Cli::parse();

    // Get the database.
    hw_msg::infoln!("Connecting to database...");

    let db_connection_string = format!(
        "mysql://{}:{}@{}/{}",
        cli.db_username, cli.db_password, cli.db_host, cli.db_name,
    );
    let db = match Database::connect(&db_connection_string).await {
        Ok(db) => db,
        Err(err) => {
            hw_msg::errorln!("Failed to connect to database. [{}]", err);
            quit::with_code(exitcode::UNAVAILABLE);
        }
    };

    // Run migrations.
    if !cli.skip_migrations {
        hw_msg::infoln!("Running database migrations...");

        if let Err(err) = Migrator::up(&db, None).await {
            hw_msg::errorln!("Failed to run database migrations. [{}]", err);
            quit::with_code(exitcode::UNAVAILABLE);
        }
    }

    // Launch the requested user command.
    let exit_code = match cli.command {
        Commands::ServeHttp { address, port, log_level} => http::start(db.clone(), address, port, log_level).await
    };
    quit::with_code(exit_code);
}
