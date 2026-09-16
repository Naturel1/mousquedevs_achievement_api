use app::auth::jwt::{create_jwt, decode_jwt};
use app::auth::password::{hash_password, verify_password};
use app::build_rocket;
use rocket::http::Status;
use rocket::local::blocking::Client;

#[test]
fn test_rocket_build_and_routes() {
    let rocket = build_rocket();
    let routes: Vec<_> = rocket.routes().map(|r| r.uri.to_string()).collect();

    // Verify presence of all required routes
    assert!(routes.iter().any(|uri| uri.contains("health")));
    assert!(routes.iter().any(|uri| uri.contains("auth")));
    assert!(routes.iter().any(|uri| uri.contains("profiles")));
    assert!(routes.iter().any(|uri| uri.contains("achievements")));
    assert!(routes.iter().any(|uri| uri.contains("admin")));
}

#[test]
fn test_password_hashing_and_verification() {
    let password = "SuperSecretPassword123!";
    let hashed = hash_password(password).expect("Password hashing must succeed");

    assert_ne!(password, hashed);
    assert!(verify_password(password, &hashed).expect("Verification must succeed"));
    assert!(!verify_password("WrongPassword", &hashed)
        .expect("Verification must succeed with negative outcome for wrong password"));
}

#[test]
fn test_jwt_generation_and_decoding() {
    let user_id = 42;
    let username = "alexandre_dumas";
    let role = "admin";

    let token = create_jwt(user_id, username, role).expect("JWT creation must succeed");
    assert!(!token.is_empty());

    let claims = decode_jwt(&token).expect("JWT decoding must succeed");
    assert_eq!(claims.sub, user_id);
    assert_eq!(claims.username, username);
    assert_eq!(claims.role, role);
}

#[test]
fn test_health_check_endpoint() {
    let rocket = rocket::build().mount("/api", rocket::routes![app::routes::health::health_check]);
    let client = Client::tracked(rocket).expect("Valid Rocket client");
    let response = client.get("/api/health").dispatch();
    assert_eq!(response.status(), Status::Ok);

    let body = response.into_string().unwrap();
    assert!(body.contains("status"));
    assert!(body.contains("ok"));
}

#[test]
fn test_logging_system_file_output() {
    app::logging::init_logger();
    log::info!("Test log message for verification suite");

    let log_file_path = std::env::var("LOG_FILE").unwrap_or_else(|_| "logs/app.log".to_string());
    assert!(std::path::Path::new(&log_file_path).exists(), "Log file must exist");

    let content = std::fs::read_to_string(&log_file_path).expect("Log file must be readable");
    assert!(content.contains("[INFO]"), "Logs must include the log level");
    assert!(content.contains("Test log message for verification suite"), "Logs must include message content");
}
