use clap::{Parser, Subcommand};
use rocket::config::LogLevel;

#[derive(Parser)]
pub struct Cli {
    /// The username to connect to the database with.
    #[arg(long, env("MPR_DATABASE_USERNAME"))]
    pub db_username: String,

    /// The password to connect to the database with.
    #[arg(long, env("MPR_DATABASE_PASSWORD"))]
    pub db_password: String,

    /// The host URL (plus optional port) of the database.
    #[arg(long, env("MPR_DATABASE_HOST"))]
    pub db_host: String,

    /// The name of the database to connect to.
    #[arg(long, env("MPR_DATABASE_NAME"))]
    pub db_name: String,

    /// Skip running migrations on startup.
    #[arg(long, env("MPR_SKIP_MIGRATIONS"))]
    pub skip_migrations: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Serve the HTTP interface.
    ServeHttp {
        /// The IP address to bind to.
        #[arg(long, default_value = "127.0.0.1", env("MPR_HTTP_ADDRESS"))]
        address: String,

        /// The port to serve over.
        #[arg(long, default_value = "8080", env("MPR_HTTP_PORT"))]
        port: u16,

        /// The amount of logging to report.
        #[arg(long, default_value = "critical")]
        log_level: LogLevel
    }
}
