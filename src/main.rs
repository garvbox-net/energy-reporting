use std::sync::Arc;

use axum::{routing::get, Router};
use dotenv::dotenv;
use envconfig::Envconfig;
use influxdb::Client;
use tokio::net::TcpListener;
use tracing::info;

mod config;
mod handlers;

pub struct AppState {
    client: Client,
}

impl AppState {
    pub fn new() -> Self {
        let config = config::Config::init_from_env().unwrap();
        let client = Client::new(config.influx_url, config.influx_db);
        Self { client }
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    // Set up InfluxDB Client and store as shared state
    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/ping", get(handlers::ping_db))
        .route("/sample_data", get(handlers::get_sample_data))
        .route("/device_summaries", get(handlers::get_device_summaries))
        .with_state(state)
        .fallback(handlers::handler_404);

    info!("Starting server...");
    let listener = TcpListener::bind("0.0.0.0:8000")
        .await
        .expect("Failed to bind listener");
    axum::serve(listener, app)
        .await
        .expect("Failed to start axum server");
}
