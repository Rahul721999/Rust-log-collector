use base62::encode;
use uuid::Uuid;

/// Generates a unique log identifier using UUID v4
///
/// # Description
/// Creates a universally unique identifier (UUID v4) for log entries.
/// UUIDs are 128-bit numbers that provide practically guaranteed uniqueness
/// across space and time.
///
/// # Returns
/// * `String` - A 36-character string representation of the UUID
///   in the format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
pub(crate) fn generate_log_id() -> String {
    let uuid = Uuid::new_v4(); // 128-bit UUID v4
    uuid.to_string()
}

/// Generates a shorter event identifier
///
/// Creates a truncated base62 encoded string from a UUID v4.
/// This provides a shorter identifier for events while maintaining sufficient
/// uniqueness for most use cases. The truncation helps keep log entries more compact.
pub fn generate_event_id() -> String {
    let uuid = Uuid::new_v4(); // 128-bit UUID v4
    encode(uuid.as_u128())
}
