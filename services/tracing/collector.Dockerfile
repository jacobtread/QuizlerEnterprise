
# Open Telemetry Collector 
FROM jaegertracing/jaeger-collector:1

ENV SPAN_STORAGE_TYPE=memory

# collector - accept OpenTelemetry Protocol (OTLP) over gRPC
EXPOSE 4317
# collector - accept OpenTelemetry Protocol (OTLP) over HTTP
EXPOSE 4318
# Admin port: health check 
EXPOSE 14269