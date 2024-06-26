use log::error;
use std::env::{self, VarError};

pub fn get_env_string(key: &str) -> Result<String, VarError> {
    return env::var(key).map_err(|e| {
        // Map the error to log it before panicking
        error!("{} is missing: {}", key, e);
        e
    });
}

pub fn get_env_i64(key: &str) -> Result<i64, VarError> {
    return env::var(key)
        .map(|val| val.parse::<i64>().expect("Value is not a valid i64"))
        .map_err(|e| {
            // Map the error to log it before panicking
            error!("{} is missing: {}", key, e);
            e
        });
}
