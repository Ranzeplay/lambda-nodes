FROM rust:1.86 AS builder
# Set the working directory
WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY src/ src/

# Build the project in release mode
RUN cargo build --release

# Create a minimal runtime image
FROM debian:bookworm-slim AS runner
WORKDIR /app

# Copy the built binary from the builder stage
COPY --from=builder /build/target/release/agent /app/agent

# Set the entrypoint
ENTRYPOINT ["/app/agent"]
