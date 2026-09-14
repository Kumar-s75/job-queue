use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn connect(url: &str, max_connections: u32) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new().max_connections(max_connections).connect(url).await
}

pub async fn migrate(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::migrate!().run(pool).await.map_err(sqlx::Error::from)
}
