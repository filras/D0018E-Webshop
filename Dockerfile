# Build backend
FROM rust:latest AS backend-builder
# RUN apt-get update && apt-get install -y default-libmysqlclient-dev && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY Cargo.toml Cargo.lock /app/
RUN mkdir /app/src && echo "fn main() {}" > /app/src/main.rs
RUN cargo build --release && rm -rf /app/src
COPY . .
RUN touch /app/src/main.rs && cargo build --release
# RUN cargo install --path .

# Build frontend
FROM node:latest AS frontend-builder
WORKDIR /app
COPY frontend/package*.json ./
RUN npm install
COPY ./frontend .
RUN npm run build

# Run backend
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y default-libmysqlclient-dev && rm -rf /var/lib/apt/lists/*
# RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=frontend-builder /app/dist /srv/frontend/dist
COPY --from=backend-builder /app/target/release/main /srv/app
WORKDIR /srv
CMD ["/srv/app"]
