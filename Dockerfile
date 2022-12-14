FROM rust:1.66 AS chef 
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN cargo install cargo-chef 
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
ARG CRATE_NAME

COPY . .
RUN cargo build --release --bin ${CRATE_NAME}

# We do not need the Rust toolchain to run the binary!
FROM debian:bullseye-slim AS runtime
RUN apt  update && apt upgrade -y
RUN apt install -y ca-certificates

FROM runtime
ARG CRATE_NAME

WORKDIR /app
COPY --from=builder /app/target/release/${CRATE_NAME} /app
ENV CRATE=${CRATE_NAME}
ENV RUST_LOG=INFO
ENTRYPOINT  "/app/${CRATE}"