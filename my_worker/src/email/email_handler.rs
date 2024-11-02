use axum::Json;
use serde_json::{json, Value};
use tracing::info;

use super::{worker, EmailTemplateParams};
use super::error::Result;

pub async fn send_email_route(
    Json(payload): Json<EmailTemplateParams>,
) -> Result<Json<Value>> {
    info!("{:<20} - {:?}\n", "ROUTE HANDLER", "send_email_route");

    // Await the send_email function and handle any potential errors.
    if let Err(e) = worker::send_email(payload.clone()).await {
        // Log the error and return a response indicating failure.
        info!("{:<20} - Sending email failed: {:?}", "ROUTE HANDLER ERROR", e);
        return Ok(Json(json!({
            "result": {
                "success": false,
                "state": "Error",
                "message": e.to_string()
            }
        })));
    }

    let body = Json(json!({
        "result": {
            "success": true,
            "state": "Ok"
        }
    }));

    Ok(body)
}

pub async fn send_vec_email_route(
    Json(payload): Json<Vec<EmailTemplateParams>>,
) -> Result<Json<Value>> {
    info!("{:<20} - {:?}\n", "ROUTE HANDLER", "send_vec_email_route");

    // Create a vector of futures for sending emails
    let futures: Vec<_> = payload.into_iter().map(|email_param| {
        // Send the email and return the future
        worker::send_email(email_param)
    }).collect();

    // Await all futures concurrently
    let results: Vec<Result<()>> = futures::future::join_all(futures).await;

    // Check if any email failed to send
    let success = results.iter().all(|result| result.is_ok());

    let body = Json(json!({
        "result": {
            "success": success,
            "state": if success { "Ok" } else { "Error" },
            "message": if success { "All emails sent successfully." } else { "Some emails failed to send." }
        }
    }));

    Ok(body)
}

