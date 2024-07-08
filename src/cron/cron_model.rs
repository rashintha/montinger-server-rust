use chrono::{DateTime, Local};

use crate::{api::monitors::monitors_model::Monitor, util::util_converters::object_id_to_i32};

#[derive(Debug, Clone)]
pub struct Cron {
    pub id: i32,
    pub _id: String,
    pub cron_expression: String,
    pub cron_type: String,
    pub last_run_time: Option<DateTime<Local>>,
    pub url: Option<String>,
}

impl From<Monitor> for Cron {
    fn from(monitor: Monitor) -> Self {
        let timestamp_millis = if let Some(last_run_time) = monitor.last_run_time {
            last_run_time.timestamp_millis()
        } else {
            0
        };

        let datetime_utc = chrono::DateTime::<chrono::Utc>::from_timestamp_millis(timestamp_millis);

        Self {
            id: object_id_to_i32(monitor.id.unwrap()),
            _id: monitor.id.map(|id| id.to_string()).unwrap_or_default(),
            cron_expression: monitor.schedule,
            cron_type: monitor._type,
            last_run_time: Some(datetime_utc.unwrap().with_timezone(&Local)),
            url: monitor.url,
        }
    }
}
