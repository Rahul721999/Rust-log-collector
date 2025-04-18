mod builder;
mod generator;
mod logs;
mod settings;

use builder::{LogAction, LogBuilder};
use generator::generate_event_id;
use logs::{log_error, log_info, log_warn};
use settings::init_logger;
use std::{thread, time};
use tracing::{info, instrument};

#[tokio::main]
async fn main() {
    init_logger("Rust-log-collector").await.unwrap();

    let threads = vec![
        spawn_profile_update_thread(),
        spawn_security_warning_thread(),
        spawn_transaction_failure_thread(),
    ];

    for (i, handle) in threads.into_iter().enumerate() {
        match handle.join() {
            Err(_) => {
                let event_id = generate_event_id();
                log_error(LogBuilder::system(
                    &event_id,
                    "LoggerService",
                    &format!("Thread {} failed to join", i),
                ));
            }
            Ok(_) => {}
        }
    }

    let event_id = generate_event_id();
    log_error(LogBuilder::system(
        &event_id,
        "LoggerService",
        "Unexpected shutdown",
    ));
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum Action {
    UserProfileUpdate,
    SecurityWarning,
    TransactionFailed,
}

impl LogAction for Action {}

impl ToString for Action {
    fn to_string(&self) -> String {
        match self {
            Action::UserProfileUpdate => "UserProfileUpdate".to_string(),
            Action::SecurityWarning => "SecurityWarning".to_string(),
            Action::TransactionFailed => "TransactionFailed".to_string(),
        }
    }
}

#[instrument(fields(event_id, user_id, module, user_name))]
fn spawn_profile_update_thread() -> thread::JoinHandle<()> {
    let user_id = "user123";
    let event_id = generate_event_id();
    let module = "UserProfileModule";
    let user_name = "john_doe";

    thread::spawn(move || {
        let interval = time::Duration::from_secs(10);
        loop {
            log_info(LogBuilder::user(
                &event_id,
                module,
                user_id,
                user_name,
                "User updated profile details",
                Action::UserProfileUpdate,
            ));
            nested_fn();
            thread::sleep(interval);
        }
    })
}

#[instrument]
fn nested_fn() {
    info!("Loggin from nested fn")
}

fn spawn_security_warning_thread() -> thread::JoinHandle<()> {
    let user_id = "user456";
    let event_id = generate_event_id();
    let module = "SecurityModule";
    let user_name = "jane_doe";

    thread::spawn(move || {
        let interval = time::Duration::from_secs(10);
        loop {
            log_warn(LogBuilder::user(
                &event_id,
                module,
                user_id,
                user_name,
                "Multiple failed login attempts",
                Action::SecurityWarning,
            ));
            thread::sleep(interval);
        }
    })
}

fn spawn_transaction_failure_thread() -> thread::JoinHandle<()> {
    let user_id = "user789";
    let event_id = generate_event_id();
    let module = "TransactionModule";
    let user_name = "alice_smith";

    thread::spawn(move || {
        let interval = time::Duration::from_secs(10);
        loop {
            log_error(LogBuilder::user(
                &event_id,
                module,
                user_id,
                user_name,
                "Transaction failed: Insufficient funds",
                Action::TransactionFailed,
            ));
            thread::sleep(interval);
        }
    })
}
