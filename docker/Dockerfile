# Step 1 : Build the application
FROM rust:1.85 AS builder

WORKDIR /usr/src/app
COPY . .

# Build in release mode to optimize size
RUN cargo build --release

# Step 2 : Minimal runtime image
FROM debian:bookworm-slim

# To run the app (add if needed, ex: ca-certificates)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/telemetry-collector /usr/local/bin/telemetry-collector

EXPOSE 8080

CMD ["telemetry-collector"]
