use log::error;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::error::Error;

pub async fn establish_connection(database_url: &str) -> Result<PgPool, Box<dyn Error>> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .map_err(|err| {
            error!("Error al conectarse a la base de datos: {}", err);
            Box::new(err) as Box<dyn Error>
        })?;

    Ok(pool)
}
