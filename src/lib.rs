pub mod builder;
pub mod generator;
pub mod setting;

use builder::{LogAction, LogBuilder};
use chrono::{DateTime, Utc};
use core::fmt;
use generator::generate_log_id;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub log_id: String,
    pub log_level: String,
    pub log_type: String,
    // pub scope: String, // will be added by the tracing_subs
    pub module: String,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub event_id: String,
    pub log_msg: String,
    pub action: String,
    pub metadata: Option<HashMap<String, String>>,
}

#[inline(always)]
pub fn create_log<A: LogAction + fmt::Debug>(log: LogBuilder<A>, log_level: &str) -> String {
    let log_id = generate_log_id();

    let log_entry = LogEntry {
        timestamp: Utc::now(),
        log_id,
        log_level: log_level.to_string(),
        log_type: log.log_type.to_string(),
        module: log.module,
        user_id: log.user_id,
        user_name: log.user_name,
        event_id: log.event_id,
        log_msg: log.log_msg,
        action: log
            .action
            .map(|a| a.to_string())
            .unwrap_or_else(|| "None".to_owned()),
        metadata: log.metadata,
    };

    match serde_json::to_string(&log_entry) {
        Ok(json) => json,
        Err(e) => format!("Failed to serialize log: {}", e),
    }
}

pub fn log_info<A: LogAction + fmt::Debug>(log: LogBuilder<A>) {
    let log_entry = create_log(log, "INFO");
    tracing::info!("{}", log_entry); // Scope & module name will be added at this point by tracing_subscriber
}

pub fn log_warn<A: LogAction + fmt::Debug>(log: LogBuilder<A>) {
    let log_entry = create_log(log, "WARN");
    tracing::warn!("{}", log_entry); // Scope & module name will be added at this point by tracing_subscriber
}

pub fn log_error<A: LogAction + fmt::Debug>(log: LogBuilder<A>) {
    let log_entry = create_log(log, "ERROR");
    tracing::error!("{}", log_entry); // Scope & module name will be added at this point by tracing_subscriber
}

pub fn log_audit<A: LogAction + fmt::Debug>(log: LogBuilder<A>) {
    let log_entry = create_log(log, "AUDIT");
    tracing::info!("{}", log_entry); // Scope & module name will be added at this point by tracing_subscriber
}
