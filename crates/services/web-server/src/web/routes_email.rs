use axum::{routing::post, Router};
use lib_web::handlers::handlers_email;

pub fn routes() -> Router {
    Router::new().route("/api/email/send", post(handlers_email::send_email_route))
}
