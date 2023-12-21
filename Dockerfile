FROM rust:1.74 AS chef 
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN apt update && apt upgrade -y
RUN apt install -y libssl-dev build-essential cmake
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
FROM debian:bookworm-slim AS runtime
RUN apt  update && apt upgrade -y
RUN apt install -y ca-certificates
ARG WITH_LIBREOFFICE
RUN if [ $WITH_LIBREOFFICE = "yes" ]; then apt update && apt upgrade -y && \
  apt install  --no-install-recommends -y libreoffice;fi

FROM runtime
ARG CRATE_NAME

RUN rm -rfv /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/${CRATE_NAME} /app

ENV CRATE=${CRATE_NAME}
ENV RUST_LOG=INFO

ENTRYPOINT  "/app/${CRATE}"
