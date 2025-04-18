use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

/// A trait representing a loggable action.
///
/// This trait ensures that any type implementing `LogAction`
/// must also implement `ToString`, allowing it to be converted into a `String`.
///
/// This is useful for logging systems where actions need to be recorded as text.
pub trait LogAction: ToString + fmt::Debug {}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogBuilder<A: LogAction + fmt::Debug> {
    pub event_id: String,
    pub log_type: String,
    pub log_msg: String,
    pub module: String,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub action: Option<A>,
    pub metadata: Option<HashMap<String, String>>,
    // pub system_name: String, // system name will be added by the tracing_subs
}

impl<A: LogAction + fmt::Debug> LogBuilder<A> {
    pub fn user(event_id: &str, module: &str, user_id: &str, user_name: &str, msg: &str, action: A) -> Self {
        Self {
            event_id: event_id.to_string(),
            module: module.to_string(),
            log_type: "User".to_string(),
            user_id: Some(user_id.to_string()),
            user_name: Some(user_name.to_string()),
            log_msg: msg.to_string(),
            action: Some(action),
            metadata: Some(HashMap::new()),
        }
    }

    pub fn add_metadata(mut self, key: &str, value: &str) -> Self {
        self.metadata
            .get_or_insert_with(HashMap::new)
            .insert(key.to_string(), value.to_string());
        self
    }
}

impl<A: LogAction + fmt::Debug> fmt::Display for LogBuilder<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{{ user_id: {:?}, action: {:?}, message: \"{}\", metadata: {:?} }}",
            self.user_id, self.action, self.log_msg, self.metadata
        )
    }
}

// Specialization for system logs (forcing `A = SystemLogs`)
impl LogBuilder<SystemLogs> {
    pub fn system(event_id: &str, module: &str, msg: &str) -> Self {
        Self {
            event_id: event_id.to_string(),
            module: module.to_string(),
            log_type: "System".to_string(),
            user_id: None,
            user_name: None,
            log_msg: msg.to_string(),
            action: Some(SystemLogs::SystemAction),
            metadata: Some(HashMap::new()),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub enum SystemLogs {
    SystemAction
}
impl ToString for SystemLogs{
    fn to_string(&self) -> String {
        match self {
           SystemLogs::SystemAction => "SystemAction".to_string()
        }
    }
}
impl LogAction for SystemLogs{}