use axum::Json;
use serde_json::{json, Value};
use tracing::info;

use super::{worker, EmailTemplateParams};
use super::error::Result;

pub async fn send_email_route(
    Json(payload): Json<EmailTemplateParams>,
) -> Result<Json<Value>>{
    info!("{:<20} - {:?}", "ROUTE HANDLER", "send_email_route");
    
    let _ = worker::send_email(payload.clone()).await;

    let body = Json(json!({
        "result": {
            "success": true,
            "state": "Ok"
        }
    }));

    Ok(body)
}
