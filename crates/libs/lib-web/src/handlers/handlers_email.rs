use axum::Json;
use serde_json::{json, Value};
use tracing::info;

use crate::error::Result;

use lib_email::{worker, EmailSendWrapper};

pub async fn send_email_route(Json(payload): Json<EmailSendWrapper>) -> Result<Json<Value>> {
    info!("{:<20} - {:?}\n", "ROUTE HANDLER", "send_email_route");

    match worker::send_email(payload.email_data).await {
        Ok(_) => {
            let response = Json(json!({
            "result": {
                "success": true,
                "state": "Ok"
            }
                }));

            Ok(response)
        }
        Err(e) => {
            let response = Json(json!({
            "result": {
                "success": false,
                "state": "Error",
                "message": e.to_string()
            }
            }));

            Ok(response)
        }
    }
}
