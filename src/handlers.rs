use std::sync::Arc;

use crate::Connections;
use axum::{extract::State, http::StatusCode, response::Json};
use influxdb::ReadQuery;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

type JsonWithResponseCode = (StatusCode, Json<Value>);

pub async fn handler_404() -> JsonWithResponseCode {
    (StatusCode::NOT_FOUND, Json(json!({"error": "Not Found"})))
}

pub async fn ping_db(State(state): State<Arc<Connections>>) -> Json<Value> {
    tracing::info!("Pinging InfluxDB");
    let ping_res = match state.client.ping().await {
        Ok(result) => {
            tracing::info!("Ping OK");
            result
        }
        Err(error) => {
            tracing::error!("Ping Error {:?}", error);
            return Json(json!({"error": error.to_string()}));
        }
    };
    Json(json!({ "server_type": ping_res.0, "version": ping_res.1 }))
}

#[derive(Serialize, Deserialize)]
struct PowerStat {
    time: String,
    value: f64,
    friendly_name: String,
}

pub async fn get_data(State(state): State<Arc<Connections>>) -> Json<Value> {
    tracing::info!("Getting data from InfluxDB");
    let query_res = state
        .client
        .json_query(ReadQuery::new(
            "SELECT friendly_name, value FROM W WHERE time >= now() - 1h",
        ))
        .await
        .map(|mut result| {
            tracing::info!("Query OK");
            result.deserialize_next::<PowerStat>().unwrap()
        });
    let results = query_res.unwrap();
    let data: Vec<PowerStat> = results.series.into_iter().flat_map(|s| s.values).collect();
    Json(json!({ "results": data }))
}
