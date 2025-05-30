# Build stage
FROM rust:bookworm as builder

WORKDIR /usr/src/app
RUN cargo init --bin

COPY Cargo.toml Cargo.lock ./

# Build dependencies - this is the caching Docker layer
RUN cargo build --release
RUN rm src/*.rs

# Copy your source code and SQLx metadata
COPY src ./src
COPY config ./config
COPY migrations ./migrations
COPY abi ./abi
COPY .sqlx ./.sqlx

# Build your application
RUN cargo build --release

# Runtime stage
FROM debian:bookworm

# Install libssl3, openssl, and ca-certificates
RUN apt-get update && apt-get install -y \
    libssl3 \
    openssl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /usr/src/app/target/release/backend /usr/local/bin/backend

# Copy necessary files
COPY --from=builder /usr/src/app/config /usr/local/bin/config
COPY --from=builder /usr/src/app/migrations /usr/local/bin/migrations
COPY --from=builder /usr/src/app/abi /usr/local/bin/abi
COPY --from=builder /usr/src/app/.sqlx /usr/local/bin/.sqlx

# Set the working directory
WORKDIR /usr/local/bin

# Command to run the application
CMD ["./backend"]
