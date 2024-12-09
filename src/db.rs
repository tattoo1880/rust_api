use crate::error_handler::CustomError;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
// use diesel_migrations::embed_migrations;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lazy_static::lazy_static;
use r2d2;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

// embed_migrations!();
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();


lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create pool")
    };
}

pub fn connection() -> Result<DbConnection,CustomError>{
    POOL.get().map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let mut conn = connection().expect("Failed to get db connection");
    //todo 数据库迁移
    // embedded_migrations::run(&conn).expect("Failed to run migrations");
    conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
}