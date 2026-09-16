use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Data, Request, Response};

/// Rocket Fairing for HTTP request and response logging
pub struct RequestLogger;

#[rocket::async_trait]
impl Fairing for RequestLogger {
    fn info(&self) -> Info {
        Info {
            name: "HTTP Request & Response Logger",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        let client_ip = request
            .client_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|| "unknown".to_string());
        log::info!(
            "--> {} {} (client: {})",
            request.method(),
            request.uri(),
            client_ip
        );
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        log::info!(
            "<-- {} {} [{}]",
            request.method(),
            request.uri(),
            response.status()
        );
    }
}
