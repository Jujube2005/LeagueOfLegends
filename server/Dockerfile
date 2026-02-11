# Build stage
FROM rust:1.75-bookworm as builder

WORKDIR /usr/src/app
COPY . .

# Install build dependencies
RUN apt-get update && apt-get install -y libpq-dev pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Build the application
RUN cargo build --release

# Final stage
FROM ubuntu:22.04

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libpq5 \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/server /app/server
RUN chmod +x /app/server

# Copy static files
COPY statics /app/statics

# Set environment variables
ENV SERVER_PORT=80
ENV STAGE=Production
ENV RUST_LOG=info
ENV RUST_BACKTRACE=1

EXPOSE 80

# Run with shell to catch formatting errors or missing libs
CMD /app/server
