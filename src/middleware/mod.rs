use std::time::SystemTime;

use rocket::{
    fairing::{Fairing, Info, Kind},
    Data, Request, Response,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct LogEntry {
    timestamp: u64,
    environment: String,
    status_code: u16,
    data: Option<Value>,
    errors: Option<Value>,
}

pub struct LoggingMiddleware;

#[rocket::async_trait]
impl Fairing for LoggingMiddleware {
    fn info(&self) -> Info {
        Info {
            name: "Logging Middleware",
            kind: Kind::Request | Kind::Response,
        }
    }

    async fn on_request(&self, request: &mut Request<'_>, _data: &mut Data<'_>) {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        let environment = std::env::var("ENV").unwrap_or_else(|_| "development".to_string());
        let log_entry = LogEntry {
            timestamp,
            environment,
            status_code: 0, // Placeholder for now
            data: None,
            errors: None,
        };
        request.local_cache(|| log_entry);
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let log_entry_ref = request.local_cache(|| LogEntry {
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64,
            environment: std::env::var("ENV").unwrap_or_else(|_| "development".to_string()),
            status_code: response.status().code,
            data: None,
            errors: None,
        });

        let mut log_entry = LogEntry {
            timestamp: log_entry_ref.timestamp,
            environment: log_entry_ref.environment.clone(),
            status_code: response.status().code, // Placeholder for now
            data: None,
            errors: None,
        };

        if response.status().code >= 400 {
            log_entry.errors = match response.body_mut().to_string().await.ok() {
                Some(errors) => serde_json::from_str(&errors.as_str()).ok(),
                None => None,
            };
        } else {
            log_entry.data = match response.body_mut().to_string().await.ok() {
                Some(data) => serde_json::from_str(&data.as_str()).ok(),
                None => None,
            };
        }

        // Modify the response
        let log_entry_json = serde_json::to_string(&log_entry).unwrap(); // Serialize to JSON
        response.set_header(rocket::http::ContentType::JSON); // Set content type to JSON
        response.set_sized_body(log_entry_json.len(), std::io::Cursor::new(log_entry_json));
    }
}
