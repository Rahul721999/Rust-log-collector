use tracing::{Event, Subscriber};
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::{
    fmt::{format::Writer, FormatEvent, FormatFields},
    layer::SubscriberExt,
};

/// Initializes tracing-subscriber with custom logging settings.
pub fn init_logger(service_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the subscriber with JSON formatting and custom event formatter
    let subscriber = tracing_subscriber::registry().with(
        tracing_subscriber::fmt::layer()
            .json()
            .with_writer(std::io::stdout)
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

use tracing::field::{Field, Visit};

struct JsonFieldVisitor {
    values: serde_json::Map<String, serde_json::Value>,
}

impl JsonFieldVisitor {
    fn new() -> Self {
        Self {
            values: serde_json::Map::new(),
        }
    }
}

impl Visit for JsonFieldVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        self.values.insert(
            field.name().to_string(),
            serde_json::Value::String(value.to_string()),
        );
    }

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
