mod config;
mod template;
mod worker;

pub use config::email_config;
pub mod email_handler;
pub mod error;

pub use template::*;
pub use worker::*;

pub fn routes() -> axum::Router {
    axum::Router::new()
        .route(
            "/api/send-email",
            axum::routing::post(email_handler::send_email_route),
        )
        .route(
            "/api/send-email/bulk",
            axum::routing::post(email_handler::send_vec_email_route),
        )
}
