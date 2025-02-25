# Build backend
FROM rust:latest AS backend-builder
WORKDIR /usr/src/app
COPY . .
ENV DATABASE_URL=mysql://root:root@localhost/webshop
RUN cargo install --path .

# Build frontend
FROM node:latest AS frontend-builder
WORKDIR /app
COPY frontend/package*.json ./
RUN npm install
COPY ./frontend .
RUN npm run build

# Run backend
FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y libmysqlclient && rm -rf /var/lib/apt/lists/*
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=frontend-builder /app/dist /usr/local/bin/app/backend/dist
COPY --from=backend-builder /usr/local/cargo/bin/app /usr/local/bin/app
CMD ["app"]
