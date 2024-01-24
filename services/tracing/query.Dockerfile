# Jaeger query frontend
FROM jaegertracing/jaeger-query:1

ENV SPAN_STORAGE_TYPE=memory

# API Endpoints and Jaeger UI
EXPOSE 16686