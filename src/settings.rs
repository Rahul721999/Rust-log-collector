use std::fs::{OpenOptions, create_dir_all};
use std::path::Path;
use std::sync::OnceLock;
use tracing::{Event, Subscriber};
use tracing_appender::non_blocking;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{
    fmt::{FormatEvent, FormatFields, format::Writer},
    layer::SubscriberExt,
};
use tracing::field::{Field, Visit};


static LOG_GUARD: OnceLock<tracing_appender::non_blocking::WorkerGuard> = OnceLock::new();

/// Initializes tracing-subscriber with custom logging settings.
pub async fn init_logger(service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let log_path = "logs/log_file.log";
    let log_dir = Path::new(log_path)
        .parent()
        .expect("Failed to determine log file directory");

    // Ensure the log directory exists
    if !log_dir.exists() {
        create_dir_all(log_dir).expect("Failed to create log directory");
    }

    // Open or create log file
    let log_file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(false)
        .open(log_path)
        .expect("Failed to create log file");

    // Get non-blocking writer and store the guard
    let (non_blocking, guard) = non_blocking::NonBlockingBuilder::default()
        .buffered_lines_limit(100)
        .lossy(false)
        .finish(log_file);

    // Store the guard to prevent premature cleanup
    if LOG_GUARD.set(guard).is_err() {
        println!("Logger already initialized");
    }

    // Initialize the subscriber with JSON formatting and custom event formatter
    let subscriber = tracing_subscriber::registry().with(
        tracing_subscriber::fmt::layer()
            .json()
            .with_writer(non_blocking)
            .without_time()
            .with_level(true)
            .event_format(CustomFormatter {
                service_name: service_name.to_string(),
            }),
    );

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");
    Ok(())
}

/// Custom JSON event formatter for tracing logs
#[derive(Clone)]
struct CustomFormatter {
    service_name: String,
}
impl<S, N> FormatEvent<S, N> for CustomFormatter
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        _ctx: &tracing_subscriber::fmt::FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> std::fmt::Result {
        // Extract structured fields from the event
        let structured_data = process_event(event);

        // Start JSON log entry
        write!(writer, "{{")?;
        write!(writer, "\"scope\":\"{}\",", self.service_name)?;

        // Serialize structured event fields
        let serialized_fields = serde_json::to_string(&structured_data).unwrap_or("{}".to_string());
        write!(writer, "\"fields\":{}", serialized_fields)?;

        // Close JSON object
        write!(writer, "}}\n")?;

        Ok(())
    }
}

// JsonFieldVisitor struct for collecting event fields
struct JsonFieldVisitor {
    values: serde_json::Map<String, serde_json::Value>,
}

impl JsonFieldVisitor {
    // Create a new JsonFieldVisitor instance
    fn new() -> Self {
        Self {
            values: serde_json::Map::new(),
        }
    }
}

// Implement Visit trait for JsonFieldVisitor
impl Visit for JsonFieldVisitor {
    // Record string fields
    fn record_str(&mut self, field: &Field, value: &str) {
        self.values.insert(
            field.name().to_string(),
            serde_json::Value::String(value.to_string()),
        );
    }

    // Record debug fields
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.values.insert(
            field.name().to_string(),
            serde_json::json!(format!("{:?}", value)),
        );
    }
}

fn process_event(event: &tracing::Event) -> serde_json::Value {
    let mut visitor = JsonFieldVisitor::new();
    event.record(&mut visitor);

    // Convert collected fields into a JSON object
    serde_json::Value::Object(visitor.values)
}
