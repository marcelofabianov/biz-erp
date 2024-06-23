use crate::enviroment::Env;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let env = Env::load();

    let database_url = env.database_url;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
