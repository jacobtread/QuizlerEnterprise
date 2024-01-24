# Use an official TimescaleDB image as the base image
FROM timescale/timescaledb:latest-pg16

# Set environment variables
ENV POSTGRES_DB=mydatabase
ENV POSTGRES_USER=myuser
ENV POSTGRES_PASSWORD=mypassword

# Copy initialization scripts to the docker-entrypoint-initdb.d directory
# COPY init.sql /docker-entrypoint-initdb.d/

# Expose the PostgreSQL default port
EXPOSE 5432