use axum::{
    extract::Form,
    response::{Html, Redirect},
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use shared::{init_db, validate_voucher, mark_voucher_used, grant_access};
use serde::Deserialize;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct AuthForm {
    voucher_code: String,
}

async fn index() -> Html<String> {
    Html(include_str!("../../templates/index.html").to_string())
}

async fn auth(Form(form): Form<AuthForm>) -> Result<Html<String>, (axum::http::StatusCode, String)> {
    // In a production environment, we'd extract the MAC address from the request
    // via a custom header from the bridge or by querying ARP for the client IP.
    // For this version, we simulate the MAC extraction.
    let mock_mac = "00:11:22:33:44:55"; 

    match validate_voucher(&form.voucher_code) {
        Ok(Some(_)) => {
            mark_voucher_used(&form.voucher_code).expect("DB Error");
            grant_access(mock_mac);
            Ok(Html("<h1>Access Granted!</h1><p>You can now browse the internet.</p>".to_string()))
        }
        _ => Err((axum::http::StatusCode::FORBIDDEN, "Invalid or used voucher code.".to_string())),
    }
}

#[tokio::main]
async fn main() {
    init_db().expect("Failed to initialize database");

    let app = Router::new()
        .route("/", get(index))
        .route("/auth", post(auth))
        .fallback_service(ServeDir::new("static"));

    let addr = SocketAddr::from(([0, 0, 0, 0], 80));
    println!("Portal running on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
