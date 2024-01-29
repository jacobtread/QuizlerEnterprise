use anyhow::{anyhow, Context};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{trace, trace::BatchConfig, Resource};
use tracing::{instrument::WithSubscriber, level_filters::LevelFilter};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{
    filter::{Directive, EnvFilter},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

const SERVICE_NAME: &str = "main-node";
const TELEMETRY_COLLECTOR_ENDPOINT: &str = "TELEMETRY_COLLECTOR_ENDPOINT";

/// Initializes the OpenTelemetry tracing
pub fn init_tracing() -> anyhow::Result<()> {
    // Telemetry collector endpoint
    let endpoint: String = std::env::var(TELEMETRY_COLLECTOR_ENDPOINT)
        .map_err(|err| anyhow!("Failed to determine telemetry endpoint: {err}"))?;

    // Create the exporter
    let otlp_exporter = opentelemetry_otlp::new_exporter()
        .tonic()
        .with_endpoint(endpoint);

    let batch_config = BatchConfig::default();

    // Create the trace config
    let trace_config = trace::config().with_resource(Resource::new([
        // Name the service
        KeyValue::new("service.name", SERVICE_NAME),
    ]));

    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_batch_config(batch_config)
        .with_trace_config(trace_config)
        .with_exporter(otlp_exporter)
        // Batch export using tokio
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .context("Creating OpenTelemetry config")?;

    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::WARN.into());

    tracing_subscriber::registry()
        .with(OpenTelemetryLayer::new(tracer))
        .with(tracing_subscriber::fmt::layer().with_filter(filter))
        .try_init()?;

    Ok(())
}
