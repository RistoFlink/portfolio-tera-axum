FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . ./
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . ./
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /app

# Install SSL certificates
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the binary
COPY --from=builder /app/target/release/portfolio-tera-axum ./

# Copy templates and static files
COPY --from=builder /app/templates ./templates
COPY --from=builder /app/static ./static

CMD ["./portfolio-tera-axum"]