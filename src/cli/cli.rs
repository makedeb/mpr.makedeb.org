use clap::{Parser, Subcommand};

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

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {}
