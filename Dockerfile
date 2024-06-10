# Use the official Rust image as the base image
FROM rust:latest AS builder

# Create a new empty shell project
RUN USER=root cargo new --bin spotify
WORKDIR /spotify

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Use the official Debian image as the base image for the runtime
FROM debian:buster-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /spotify/target/release/spotify /usr/local/bin/spotify

# Expose port 7860
EXPOSE 7860

# Run the application
CMD ["spotify"]
