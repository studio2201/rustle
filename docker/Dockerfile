# syntax = docker/dockerfile:1

# Stage 1: Build the Rust WASM application
FROM rust:1.78-slim AS builder

# Install system dependencies (including nodejs and npm for tailwindcss)
RUN apt-get update && apt-get install -y pkg-config libssl-dev curl gnupg && \
    curl -fsSL https://deb.nodesource.com/setup_18.x | bash - && \
    apt-get install -y nodejs && \
    rm -rf /var/lib/apt/lists/*

# Install trunk and wasm32 compilation target
RUN cargo install --locked trunk && \
    rustup target add wasm32-unknown-unknown

WORKDIR /app

# Install npm packages for tailwindcss execution
COPY package.json package-lock.json ./
RUN npm ci

# Copy codebase and compile release artifacts
COPY . .
RUN trunk build --release

# Stage 2: Production Web Server
FROM nginx:1.24.0-alpine AS prod

# Copy custom non-root nginx configurations
COPY docker/etc/nginx/nginx.conf /etc/nginx/nginx.conf
COPY docker/etc/nginx/conf.d/default.conf /etc/nginx/conf.d/default.conf

# Copy compiled static assets
COPY --from=builder /app/dist /usr/share/nginx/html

# Configure permissions for non-root user execution
RUN addgroup -S reactle && adduser -S reactle -G reactle && \
    chown -R reactle:reactle /usr/share/nginx/html /var/cache/nginx /var/log/nginx /etc/nginx && \
    touch /tmp/nginx.pid && chown reactle:reactle /tmp/nginx.pid

EXPOSE 4409
USER reactle

CMD ["nginx", "-g", "daemon off;"]