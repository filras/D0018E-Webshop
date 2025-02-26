# Build backend
FROM rust:latest AS backend-builder
WORKDIR /app
COPY Cargo.toml Cargo.lock /app/
RUN mkdir /app/src && echo "fn main() {}" > /app/src/main.rs
RUN cargo build --release && rm -rf /app/src
COPY . .
RUN touch /app/src/main.rs && cargo build --release
# Build diesel_cli binary
RUN cargo install diesel_cli --no-default-features --features mysql

# Build frontend
FROM node:latest AS frontend-builder
WORKDIR /app
COPY frontend/package*.json ./
RUN npm install
COPY ./frontend .
RUN npm run build

# Setup container for backend
FROM debian:bookworm-slim AS server
RUN apt-get update && apt-get install -y default-libmysqlclient-dev && rm -rf /var/lib/apt/lists/*
# Copy in frontend files and server binary
COPY --from=frontend-builder /app/dist /srv/frontend/dist
COPY --from=backend-builder /app/target/release/main /srv/app
# Install diesel cli from built binary
COPY --from=backend-builder /usr/local/cargo/bin/diesel /bin/diesel
COPY diesel.toml /
COPY ./migrations /migrations
ENV DATABASE_URL mysql://root:root@database/webshop

# Run the diesel migration, then start the backend
WORKDIR /srv
CMD ["/bin/bash", "-c", "diesel migration run && /srv/app"]
