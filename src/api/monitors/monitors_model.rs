use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Monitor {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    #[serde(rename = "type")]
    pub _type: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<String>,
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
}

impl From<Monitor> for MonitorResponse {
    fn from(monitor: Monitor) -> Self {
        Self {
            id: monitor.id.map(|id| id.to_string()).unwrap_or_default(),
            _type: monitor._type,
            name: monitor.name,
            url: monitor.url,
            request: monitor.request,
        }
    }
}