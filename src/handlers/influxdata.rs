use axum::{extract::State, response::Json};
use influxdb::ReadQuery;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::sync::Arc;

use crate::AppState;

pub async fn ping_db(State(state): State<Arc<AppState>>) -> Json<Value> {
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

pub async fn get_sample_data(State(state): State<Arc<AppState>>) -> Json<Value> {
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

#[derive(Serialize, Deserialize)]
struct DevicePowerStat {
    friendly_name: String,
    energy: Decimal,
    cost: Decimal,
}

pub async fn get_device_summaries(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Value>, Json<Value>> {
    let mut query_res = state
        .client
        .json_query(ReadQuery::new(
            "SELECT max(value) \
                FROM kWh WHERE (entity_id =~ /energy_total/) AND \
                time >= now() - 1h  AND time <= now() \
                GROUP BY time(1h), friendly_name fill(linear)",
        ))
        .await
        .map_err(|e| Json(json!({"error": format!("{}", e)})))?;

    let data: Vec<DevicePowerStat> = query_res
        .deserialize_next::<DevicePowerStat>()
        .map_err(|e| Json(json!({"error": format!("{}", e)})))?
        .series
        .into_iter()
        .flat_map(|s| s.values)
        .collect();
    Ok(Json(json!({"results": data})))
}
