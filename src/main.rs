mod models;
mod routes;

use axum::{
    Router,
    routing::get,
    routing::post,
    extract::Json, 
};
use log::info;
use std::net::SocketAddr;
use tokio;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use routes::{
    index,
    user,
    message,
};

use models::response::Response;



#[derive(OpenApi)]
#[openapi(
    paths(
        index::index,
        user::get_user,
        user::create_user,
        message::list_message,
        message::create_message,
        health_checker_handler,
    ),
    tags(
        (name = "user", description = "User endpoints"),
        (name = "message", description = "Message endpoints"),
        (name = "healthchecker", description = "utils endpoints"),
        (name = "index", description = "index"),
    )
)]
struct ApiDoc;

#[utoipa::path(
    get,
    path = "/health",
    tag = "healthchecker",
    responses(
        (status = 200, description = "get current health status", body = Response)
    )
)]
async fn health_checker_handler() -> Json<Response> {
    Json(Response{
        status_code: 200,
        message: "Build Simple CRUD API in Rust using Axum".into(),
        content: "kebab".into(),
    })
}


#[tokio::main]
async fn main() {
    // 1. Initialize tracing + log bridging
    tracing_subscriber::fmt()
        // This allows you to use, e.g., `RUST_LOG=info` or `RUST_LOG=debug`
        // when running the app to set log levels.
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .or_else(|_| EnvFilter::try_new("INFO"))
                .unwrap(),
        )
        .init();

    // 2. Build our router
    let app = Router::new()
        .route("/", get(index::index))
        .route("/message", get(message::list_message).post(message::create_message))
        .route("/user", get(user::get_user).post(user::create_user))

        .route("/health", get(health_checker_handler))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()))
        
        // 3. Add a TraceLayer to automatically create and enter spans
        .layer(TraceLayer::new_for_http());

    // 4. Run our Axum server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on {}", addr);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
