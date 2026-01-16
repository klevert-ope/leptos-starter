# Build stage
FROM rust:1.92.0-bullseye AS builder

# Install trunk for building Leptos CSR apps
RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app
COPY . .

# Build the app
RUN trunk build --release

# Production stage
FROM nginx:alpine  AS production

# Copy built files from builder
COPY --from=builder /app/dist /usr/share/nginx/html

# Copy nginx configuration
COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]