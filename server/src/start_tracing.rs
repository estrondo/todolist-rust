use opentelemetry::trace::TracerProvider;
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, SpanExporter, WithExportConfig};
use opentelemetry_sdk::{Resource, logs::SdkLoggerProvider, trace::SdkTracerProvider};
use todolist_server::configuration::{Configuration, Mode};
use tracing::Level;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::Layer;
use tracing_subscriber::filter::Targets;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{Registry, util::SubscriberInitExt};

const APP_NAME: &str = "todolist-server";

pub(crate) async fn start_tracing(configuration: &Configuration, _mode: &Mode) {
    let resource = Resource::builder().with_service_name(APP_NAME).build();
    let grpc_endpoint = configuration.otel.grpc_endpoint.to_owned(); // Make it configurable

    let span_exporter = SpanExporter::builder()
        .with_tonic()
        .with_endpoint(grpc_endpoint.to_owned()) // TODO: Configurable
        .build()
        .unwrap();

    let span_trace_provider = SdkTracerProvider::builder()
        .with_batch_exporter(span_exporter)
        .with_resource(resource.to_owned())
        .build();

    let span_layer =
        tracing_opentelemetry::layer().with_tracer(span_trace_provider.tracer(APP_NAME));

    let log_exporter = LogExporter::builder()
        .with_tonic()
        .with_endpoint(grpc_endpoint)
        .build()
        .unwrap();

    let log_provider = SdkLoggerProvider::builder()
        .with_batch_exporter(log_exporter)
        .with_resource(resource)
        .build();

    let log_layer = OpenTelemetryTracingBridge::new(&log_provider);

    let otel_filter = Targets::new()
        .with_target("migration", Level::TRACE)
        .with_target("todolist_core", Level::TRACE)
        .with_target("todolist_persistence_postgres", Level::TRACE)
        .with_target("todolist_server", Level::TRACE)
        .with_default(LevelFilter::WARN);

    let fmt_filter = Targets::new().with_default(LevelFilter::INFO);

    Registry::default()
        .with(layer().with_filter(fmt_filter))
        .with(span_layer.with_filter(otel_filter.to_owned()))
        .with(log_layer.with_filter(otel_filter))
        .init();
}
