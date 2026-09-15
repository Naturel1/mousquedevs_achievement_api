pub mod achievements;
pub mod admin;
pub mod auth;
pub mod health;
pub mod profiles;

use rocket::{options, routes, Route};

/// Catch-all route to handle HTTP OPTIONS requests (CORS preflight) sent by browsers/frontends
#[options("/<_..>")]
pub fn options_preflight() {
    // Body is empty; CORS headers are automatically injected by the Cors fairing
}

/// Returns global application routes
pub fn global_routes() -> Vec<Route> {
    routes![options_preflight]
}
