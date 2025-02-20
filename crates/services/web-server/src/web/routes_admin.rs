use axum::routing::post;
use axum::Router;
use lib_core::model::ModelManager;
use lib_web::handlers::handlers_admin;

pub fn routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/admin/create", post(handlers_admin::api_create_admin))
        .route("/api/admin/remove", post(handlers_admin::api_remove_admin))
        .with_state(mm)
}
