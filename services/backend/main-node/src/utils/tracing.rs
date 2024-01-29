use tracing::level_filters::LevelFilter;
use tracing_subscriber::{filter::EnvFilter, layer::SubscriberExt, util::SubscriberInitExt, Layer};

/// Initializes the OpenTelemetry tracing
pub fn init_tracing() -> anyhow::Result<()> {
    let filter = EnvFilter::from_default_env().add_directive(LevelFilter::WARN.into());

    #[cfg(all(feature = "tracing-opentelemetry", not(feature = "tracing-console")))]
    {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().with_filter(filter))
            .with(create_open_telemetry())
            .try_init()?;
    }

    #[cfg(all(feature = "tracing-console", not(feature = "tracing-opentelemetry")))]
    {
        tracing_subscriber::registry()
            .with(tracing_subscriber::fmt::layer().with_filter(filter))
            .with(console_subscriber::spawn())
            .try_init()?;
    }

    Ok(())
}

/// Creates the open telemetry tracing layer
#[cfg(feature = "tracing-opentelemetry")]
fn create_open_telemetry() -> anyhow::Result<tracing_opentelemetry::OpenTelemetryLayer<_, _>> {
    use anyhow::{anyhow, Context};
    use opentelemetry::KeyValue;
    use opentelemetry_otlp::WithExportConfig;
    use opentelemetry_sdk::{
        trace::{self, BatchConfig},
        Resource,
    };
    use tracing_opentelemetry::OpenTelemetryLayer;

    const SERVICE_NAME: &str = "main-node";
    const TELEMETRY_COLLECTOR_ENDPOINT: &str = "TELEMETRY_COLLECTOR_ENDPOINT";

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

    Ok(OpenTelemetryLayer::new(tracer))
}
