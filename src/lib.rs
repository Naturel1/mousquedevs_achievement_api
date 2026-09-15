pub mod auth;
pub mod config;
pub mod db;
pub mod errors;
pub mod fairings;
pub mod models;
pub mod repositories;
pub mod routes;
pub mod schema;

use rocket::{Build, Rocket};

/// Builds and configures the Rocket API instance:
/// - PostgreSQL / Diesel connection pool
/// - Automatic database migrations at startup (MigrationFairing)
/// - CORS Fairing for frontend communication
/// - Standardized JSON error catchers
/// - Hierarchical mounting of all REST API routes
pub fn build_rocket() -> Rocket<Build> {
    rocket::build()
        // Attach Rocket-managed connection pool for PostgreSQL
        .attach(db::DbConn::fairing())
        // Attach fairing for automatic execution of Diesel migrations
        .attach(db::MigrationFairing)
        // Attach CORS fairing to allow cross-origin frontend requests
        .attach(fairings::cors::Cors)
        // Register HTTP error handlers (400, 401, 403, 404, 422, 500) formatted as JSON
        .register("/", errors::catchers::all_catchers())
        // Global catch-all route for CORS preflight (OPTIONS)
        .mount("/", routes::global_routes())
        // API Health Check route
        .mount("/api", rocket::routes![routes::health::health_check])
        // Authentication routes (Register, Login, Current Profile)
        .mount("/api/auth", routes::auth::routes())
        // User profile routes
        .mount("/api/profiles", routes::profiles::routes())
        // Achievement routes (View, Propose, Unlock, CRUD)
        .mount("/api/achievements", routes::achievements::routes())
        // Admin panel routes (Moderation, Role management, Audit logs)
        .mount("/api/admin", routes::admin::routes())
}
