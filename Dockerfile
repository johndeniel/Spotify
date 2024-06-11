# Start with the official Rust image
FROM rust:1.71-slim-buster AS builder

# Install required dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a new empty shell project
RUN USER=root cargo new --bin spotify
WORKDIR /spotify

# Copy the Cargo.toml and Cargo.lock if it exists
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# Copy the source code
COPY src src

# Build the dependencies only
RUN cargo build --release

# Copy the source code again to ensure it's up to date
COPY . .

# Build the application
RUN cargo build --release

# Use a minimal base image for the final stage
FROM debian:buster-slim

# Install required runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the built binary from the builder stage
COPY --from=builder /spotify/target/release/spotify /usr/local/bin/spotify

# Copy the static files to the final image
COPY src/static /usr/local/bin/static

# Copy environment variables from build arguments to the runtime environment
ARG SPOTIFY_CLIENT_ID
ARG SPOTIFY_CLIENT_SECRET
ARG SPOTIFY_PLAYLIST_ID

ENV SPOTIFY_CLIENT_ID=$SPOTIFY_CLIENT_ID
ENV SPOTIFY_CLIENT_SECRET=$SPOTIFY_CLIENT_SECRET
ENV SPOTIFY_PLAYLIST_ID=$SPOTIFY_PLAYLIST_ID

# Expose the port the app runs on
EXPOSE 7860

# Set the default command to run the application
CMD ["spotify"]
