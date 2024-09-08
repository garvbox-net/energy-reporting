use axum::{http::StatusCode, response::Json};
use serde_json::{json, Value};
mod influxdata;

pub use influxdata::{get_device_summaries, get_sample_data, ping_db};

pub async fn handler_404() -> (StatusCode, Json<Value>) {
    (StatusCode::NOT_FOUND, Json(json!({"error": "Not Found"})))
}
