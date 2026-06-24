# Stage 1: Build the Yew WASM frontend
FROM rust:1.96-alpine AS frontend-builder
RUN apk add --no-cache musl-dev wget tar curl libc6-compat
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown

# Download and install precompiled musl Trunk
RUN wget -qO- "https://github.com/trunk-rs/trunk/releases/download/v0.21.14/trunk-x86_64-unknown-linux-musl.tar.gz" | tar -xzf- -C /usr/local/bin

# Install Node.js, npm, and Tailwind CSS CLI for Alpine compatibility
RUN apk add --no-cache nodejs npm && \
    npm install -g tailwindcss@3.4.17

COPY . .
WORKDIR /app/frontend
RUN trunk build --release

# Stage 2: Build the Axum server backend
FROM rust:1.96-alpine AS backend-builder
RUN apk add --no-cache musl-dev
WORKDIR /app
COPY . .
# Copy built static files from frontend stage into dist
COPY --from=frontend-builder /app/frontend/dist /app/dist
RUN cargo build --release --bin server

# Stage 3: Slim Runner
FROM alpine:latest
LABEL org.opencontainers.image.source="https://github.com/UberMetroid/rustle"
WORKDIR /app

# Install runtime dependencies (ca-certificates for external updates, wget for health checks)
RUN apk add --no-cache ca-certificates wget libc6-compat

# Copy compiled server binary and compiled static assets
COPY --from=backend-builder /app/target/release/server /app/server
COPY --from=frontend-builder /app/frontend/dist /app/dist

RUN chown -R 99:100 /app

# Run as Unraid nobody:users
USER 99:100

EXPOSE 4409
ENV PORT=4409
CMD ["/app/server"]
