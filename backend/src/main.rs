use std::sync::Arc;
use std::time::Duration;

use axum::error_handling::HandleErrorLayer;
use axum::extract::State;
use axum::http::{self, HeaderValue, Method, StatusCode};
use axum::response::Json;
use axum::routing::get;
use axum::Router;
use common::model::repo::Repo;
use tower::{BoxError, ServiceBuilder};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeFile;
use tower_http::trace::TraceLayer;

use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

mod github;

use crate::github::GitHubClient;

type GitHubClientState = Arc<GitHubClient>;

async fn repos(
    State(client): State<GitHubClientState>,
) -> Result<Json<Vec<Repo>>, (StatusCode, String)> {
    let result = client
        .repos()
        .await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;
    Ok(Json(result))
}

#[tokio::main]
async fn main() {
    let github_client = github::GitHubClient::new().unwrap();
    let client_state = Arc::new(github_client);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/repos", get(repos))
        .route_service("/resume", ServeFile::new("assets/resume.json"))
        // Add middleware to all routes
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(|error: BoxError| async move {
                    if error.is::<tower::timeout::error::Elapsed>() {
                        Ok(StatusCode::REQUEST_TIMEOUT)
                    } else {
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Unhandled internal error: {error}"),
                        ))
                    }
                }))
                .timeout(Duration::from_secs(10))
                .layer(TraceLayer::new_for_http())
                .layer(
                    CorsLayer::new()
                        .allow_origin("http://127.0.0.1:8080".parse::<HeaderValue>().unwrap())
                        .allow_methods([Method::GET])
                        .allow_headers([http::header::CONTENT_TYPE]),
                )
                .into_inner(),
        )
        .with_state(client_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
