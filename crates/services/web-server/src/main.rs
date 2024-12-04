use axum::{middleware, routing::get, Json, Router};
use lib_core::model::ModelManager;
use lib_web::{
    middleware::{
        mw_auth::mw_ctx_resolver, mw_req_stamp::mw_req_stamp_resolver, mw_res_map::mw_reponse_map,
    },
    routes::route_static,
};
use serde_json::{json, Value};
use tower_cookies::CookieManagerLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;
use web::{routes_admin, routes_login};

mod config;
mod error;
mod web;

pub use self::error::{Error, Result};
use config::web_config;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_target(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let mm = ModelManager::new().await?;

    let app = Router::new()
        .merge(routes_login::routes(mm.clone()))
        .merge(routes_admin::routes(mm.clone()))
        .route("/api/greetings", get(greetings))
        .layer(middleware::map_response(mw_reponse_map))
        .layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolver))
        .layer(CookieManagerLayer::new())
        .layer(middleware::from_fn(mw_req_stamp_resolver))
        .fallback_service(route_static::serve_dir(&web_config().WEB_FOLDER));

    let listener = tokio::net::TcpListener::bind(&web_config().SERVICE_URL)
        .await
        .unwrap();

    info!("{:<12} - {:?}\n", "SERVER LISTENING", listener.local_addr());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

pub async fn greetings() -> Json<Value> {
    info!("{:<12} - {:?}\n", "ROUTE TESTING", "greetings");

    Json(json!({
    "greetings": "Hello world",
    "success": 200
    }))
}
