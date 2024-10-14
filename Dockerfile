FROM rust:1.81 AS chef 
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN apt-get update && apt-get upgrade -y
RUN apt-get install -y libssl-dev build-essential cmake
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
RUN apt  update && apt-get upgrade -y
RUN apt-get install -y ca-certificates
ARG WITH_LIBREOFFICE
ARG WITH_CHROMIUM
RUN if [ $WITH_LIBREOFFICE = "yes" ]; then apt-get update && apt-get upgrade -y && \
  apt-get install  --no-install-recommends -y libreoffice;fi
RUN if [ $WITH_CHROMIUM = "yes" ]; then apt-get update && apt-get install -y \
  chromium \
  --no-install-recommends;fi

FROM runtime
ARG CRATE_NAME
ARG USERNAME=${CRATE_NAME}
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user, enable this once volume are properly mapped
# RUN groupadd --gid $USER_GID $USERNAME \
#     && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME 
# RUN rm -rfv /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/${CRATE_NAME} /app

ENV CRATE=${CRATE_NAME}
ENV RUST_LOG=INFO
# USER $USERNAME
ENTRYPOINT  "/app/${CRATE}"
