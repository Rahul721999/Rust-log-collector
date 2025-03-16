use std::{thread, time};
use tracing::{error, info, warn};
use tracing_subscriber::fmt::format::FmtSpan;

fn main() {
    // Initialize Logger with JSON output
    tracing_subscriber::fmt()
        .pretty()
        .with_span_events(FmtSpan::CLOSE)
        // .with_current_span(false)
        .init();

    let user_id = "123e4567-e89b-12d3-a456-426614174000";

    let mut threads = Vec::new();

    // Thread 1: Logs user profile updates every 5 seconds
    threads.push(thread::spawn({
        let user_id = user_id;
        move || {
            let interval = time::Duration::from_secs(5);
            loop {
                info!(
                    user_id = user_id,
                    action = "Updated Profile",
                    entity = %EntityType::UserProfileUpdate,
                    "User updated profile details"
                );
                thread::sleep(interval);
            }
        }
    }));

    // Thread 2: Logs security warnings every 6 seconds
    threads.push(thread::spawn({
        let user_id = user_id;
        move || {
            let interval = time::Duration::from_secs(6);
            loop {
                warn!(
                    user_id = user_id,
                    action = "Security Warning",
                    entity = %EntityType::Authentication,
                    "Multiple failed login attempts"
                );
                thread::sleep(interval);
            }
        }
    }));

    // Thread 3: Logs transaction failures every 7 seconds
    threads.push(thread::spawn({
        let user_id = user_id;
        move || {
            let interval = time::Duration::from_secs(7);
            loop {
                error!(
                    user_id = user_id,
                    action = "Transaction Failed",
                    entity = %EntityType::Transaction,
                    reason = "Insufficient funds",
                    "Transaction failed"
                );
                thread::sleep(interval);
            }
        }
    }));

    for (i, handle) in threads.into_iter().enumerate() {
        match handle.join() {
            Err(_) => error!(
                thread_id = i,
                service = "LoggerService",
                error = "Thread failed to join",
                "System thread encountered an error."
            ),
            Ok(_) => {}
        }
    }

    error!(
        service = "LoggerService",
        reason = "Unexpected shutdown",
        "System shutting down unexpectedly"
    );
}

// Role Enum
#[derive(Debug, Clone, Copy)]
pub enum Role {
    Admin,
    User,
    Customer,
}

// EntityType Enum
#[derive(Debug, Clone, Copy)]
pub enum EntityType {
    Authentication,
    Transaction,
    UserProfileUpdate,
}

// Implement Display trait for Role
impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

// Implement Display trait for EntityType
impl std::fmt::Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
