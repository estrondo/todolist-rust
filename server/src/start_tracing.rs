use opentelemetry::trace::TracerProvider;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, SpanExporter, WithExportConfig};
use opentelemetry_sdk::{Resource, logs::SdkLoggerProvider, trace::SdkTracerProvider};
use todolist_server::configuration::Configuration;
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub(crate) fn with(_configuration: &Configuration) {
    fn create_resource() -> Resource {
        Resource::builder()
            .with_service_name("todolist-server")
            .build()
    }

    fn create_span_exporter() -> SpanExporter {
        opentelemetry_otlp::SpanExporter::builder()
            .with_http()
            .with_protocol(opentelemetry_otlp::Protocol::Grpc)
            .build()
            .unwrap()
    }

    fn create_log_exporter() -> LogExporter {
        opentelemetry_otlp::LogExporter::builder()
            .with_http()
            .with_protocol(opentelemetry_otlp::Protocol::Grpc)
            .build()
            .unwrap()
    }

    let tracer_provider = SdkTracerProvider::builder()
        .with_batch_exporter(create_span_exporter())
        .with_resource(create_resource())
        .build();

    let tracer = tracer_provider.tracer("todolist-server");
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // configuring loging...
    let log_provider = SdkLoggerProvider::builder()
        .with_batch_exporter(create_log_exporter())
        .with_resource(create_resource())
        .build();

    let layer = OpenTelemetryTracingBridge::new(&log_provider);

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();

    subscriber.with(telemetry).with(layer).init();
}
