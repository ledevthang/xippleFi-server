mod error;
mod extractors;
mod handlers;
mod middlewares;
mod openapi;

use axum::{
    routing::{get, post},
    Router,
};
use extractors::states::AppState;
use handlers::{assets::*, auth::*, users::*};
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("failed to load env");

    let filter = EnvFilter::builder()
        .from_env()
        .unwrap()
        .add_directive("server=debug".parse().unwrap());

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let db_url = std::env::var("DATABASE_URL").expect("missing DATABASE_URL");
    let redis_url = "redis://127.0.0.1/";

    let state = AppState::new(&db_url, redis_url)
        .await
        .expect("fail to init state");

    let assets_router = Router::new()
        .route("/api/assets/:asset_id", get(get_asset))
        .route("/api/assets", get(get_assets));

    let auth_router = Router::new()
        .route("/api/auth/message", get(get_message))
        .route("/api/auth/verify-signature", post(verify_signature));

    let user_router = Router::new().route("/api/user/assets", get(get_user_assets));

    let app = Router::new()
        .route("/api", get(|| async { "hello Xipple !" }))
        .merge(
            SwaggerUi::new("/api/docs").url("/api-docs/openapi.json", openapi::ApiDoc::openapi()),
        )
        .merge(auth_router)
        .merge(user_router)
        .merge(assets_router)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    tracing::info!("🦀 server listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}