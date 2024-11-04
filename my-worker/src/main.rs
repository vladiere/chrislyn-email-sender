use axum::{routing::get, Json, Router};
use my_worker::{core_config, email, error::Result};
use tracing::info;
use serde_json::{json, Value};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let routes_all = Router::new()
        .route("/", get(greetings))
        .merge(email::routes());
        // .fallback_service(routes_static::serve_dir())

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", &core_config().SERVER_HOST, &core_config().SERVER_PORT)).await.unwrap();
    
    info!("{:<20} - {:?}\n", "LISTENING", listener.local_addr());

    axum::serve(listener, routes_all.into_make_service()).await.unwrap();

    Ok(())
}

async fn greetings() -> Json<Value> {
    info!("{:<20} - {:?}\n", "ROUTE TESTING", "greetings");

    Json(json!({
        "SUCCESS": true,
        "MESSAGE": "Hello world"
    }))
}
