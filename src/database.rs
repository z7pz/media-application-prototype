use crate::{DB_URL, POOL};

pub async fn init_db() {
    let options = sqlx::postgres::PgPoolOptions::new()
        .max_lifetime(None)
        .idle_timeout(None);

    let pool = options
        .connect(&DB_URL) 
        .await
        .expect("couldn't connect to database!");
    sqlx::migrate!("db/migrations")
        .run(&pool)
        .await
        .expect("Failed to run the migration");
    POOL.set(pool).expect("couldn't asign the pool to global");
}
