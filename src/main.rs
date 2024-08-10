use crate::app::start_server;
use crate::db::establish_connection;
use dotenvy::dotenv;
use log::error;
use std::process;

mod app;
mod db;
mod handlers;
mod persistance;
mod models;
mod routes;

use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        error!("DATABASE_URL no está definido en el archivo .env");
        process::exit(1);
    });

    let pool = establish_connection(&database_url).await?;
    start_server(pool).await?;

    Ok(())
}
