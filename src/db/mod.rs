use diesel::pg::PgConnection;
use diesel::Connection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use rocket_sync_db_pools::database;

/// Constant embedding all SQL migration files from the `migrations/` directory into the compiled binary
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

/// PostgreSQL connection pool managed by Rocket via Diesel
/// The name "postgres_db" corresponds to the key configured in Rocket.toml and docker-compose.yml
#[database("postgres_db")]
pub struct DbConn(PgConnection);

/// Runs pending Diesel migrations against the specified database URL
pub fn run_embedded_migrations(
    database_url: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let mut conn = PgConnection::establish(database_url)?;
    conn.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

/// Rocket Fairing to automatically apply Diesel migrations on server startup
pub struct MigrationFairing;

#[rocket::async_trait]
impl rocket::fairing::Fairing for MigrationFairing {
    fn info(&self) -> rocket::fairing::Info {
        rocket::fairing::Info {
            name: "Execute Diesel Database Migrations",
            kind: rocket::fairing::Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: rocket::Rocket<rocket::Build>) -> rocket::fairing::Result {
        let db_url = rocket
            .figment()
            .extract_inner::<String>("databases.postgres_db.url")
            .or_else(|_| std::env::var("DATABASE_URL"))
            .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/maadb".to_string());

        match run_embedded_migrations(&db_url) {
            Ok(_) => {
                log::info!("Diesel migrations successfully applied");
                Ok(rocket)
            }
            Err(e) => {
                // If the database is unreachable (e.g. quick unit tests), warn without blocking
                log::warn!("Non-blocking migration attempt: {}", e);
                Ok(rocket)
            }
        }
    }
}
