use crate::environment::Environment as Env;
use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect(env: &Env) -> Result<PgPool, sqlx::Error> {
    let database_url = &env.database_url;

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    Ok(pool)
}
