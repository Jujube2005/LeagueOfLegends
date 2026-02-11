# Build stage for Rust server
FROM rust:1.75-slim-bookworm as builder

WORKDIR /usr/src/app
COPY . .

# Install dependencies for diesel (libpq for postgres)
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y libpq-dev ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/server /app/server

# Copy static files (Frontend build should be put here before building image or via another stage)
COPY statics /app/statics

# Set env vars defaults
ENV SERVER_PORT=80
ENV STAGE=Production

EXPOSE 80

CMD ["./server"]
