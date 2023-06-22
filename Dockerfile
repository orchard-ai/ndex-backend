# Stage 1: Building the code
FROM rust:1.70.0 as builder

WORKDIR /usr/src

# Create a new empty shell project
RUN USER=root cargo new ndex-backend
WORKDIR /usr/src/ndex-backend

# Copy over your Manifest files
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Build dependencies - this will cache dependencies and help speed up subsequent builds
RUN cargo build --release
RUN rm src/*.rs

COPY ./migrations ./migrations
COPY ./.env ./.env

# Copy the source code
COPY ./src ./src

# Build the application
RUN rm ./target/release/deps/*
RUN cargo build --release

# Stage 2: Preparing the final image
FROM debian:buster-slim

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from the builder stage
COPY --from=builder /usr/src/ndex-backend/target/release/ndex-backend /usr/local/bin

# Set the startup command to run your binary
CMD ["ndex-backend"]
