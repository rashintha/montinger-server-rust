use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Monitor {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    pub status: String,
    pub schedule: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_time: Option<DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_data: Option<Vec<MonitorData>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitorData {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_id: Option<ObjectId>,
    pub time: DateTime,
    pub response_time: i64,
    pub status_code: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitorResponse {
    id: String,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_run_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history_data: Option<Vec<MonitorDataResponse>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MonitorDataResponse {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_id: Option<ObjectId>,
    pub time: String,
    pub response_time: i64,
    pub status_code: i64,
}

impl From<Monitor> for MonitorResponse {
    fn from(monitor: Monitor) -> Self {
        Self {
            id: monitor.id.map(|id| id.to_string()).unwrap_or_default(),
            _type: monitor._type,
            name: monitor.name,
            url: monitor.url,
            request: monitor.request,
            status: monitor.status,
            last_run_time: monitor
                .last_run_time
                .map(|last_run_at| last_run_at.to_string()),
            history_data: monitor.history_data.map(|history_data| {
                history_data
                    .into_iter()
                    .map(|monitor_data| MonitorDataResponse::from(monitor_data))
                    .collect()
            }),
        }
    }
}

impl From<MonitorData> for MonitorDataResponse {
    fn from(monitor_data: MonitorData) -> Self {
        Self {
            id: monitor_data.id,
            monitor_id: monitor_data.monitor_id,
            time: monitor_data.time.to_string(),
            response_time: monitor_data.response_time,
            status_code: monitor_data.status_code,
        }
    }
}
