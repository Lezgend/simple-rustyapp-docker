# Step 1: Build the Rust project
FROM rust:alpine AS builder

# Install necessary dependencies
RUN apk add --no-cache musl-dev sudo bash curl wget

# Install wasm32 target for Rust
RUN rustup target add wasm32-unknown-unknown

# Install trunk and wasm-bindgen-cli
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall trunk wasm-bindgen-cli

# Set the working directory
WORKDIR /app

# Copy the current project into the container
COPY . .

# Download the miniserve binary and make it executable
RUN wget https://github.com/svenstaro/miniserve/releases/download/v0.28.0/miniserve-0.28.0-aarch64-unknown-linux-musl -O miniserve && chmod +x miniserve

# Build the application for the wasm32 target (using trunk to compile frontend)
RUN trunk build --release

# Step 2: Final minimal image with scratch
FROM scratch

# Copy the compressed asmttpd binary from the builder stage
COPY --from=builder /app/miniserve /miniserve

# Copy the compiled static files from the builder stage
COPY --from=builder /app/dist /web_root

# Expose port 80
EXPOSE 80

# Set the entrypoint to start the web server
ENTRYPOINT ["/miniserve"]

# Default command to serve files from /web_root
CMD ["/web_root", "--port", "80", "--index", "index.html"]
