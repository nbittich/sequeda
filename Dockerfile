FROM rust:1.74 AS chef 
# We only pay the installation cost once, 
# it will be cached from the second build onwards
RUN apt update && apt upgrade -y
RUN apt install -y libssl-dev build-essential cmake
RUN cargo install cargo-chef 

# just to build
ENV MAGICK_VERSION 7.1

RUN apt -y install curl build-essential clang pkg-config libjpeg-turbo-progs libpng-dev \
  && curl https://imagemagick.org/archive/ImageMagick.tar.gz | tar xz \
  && cd ImageMagick-${MAGICK_VERSION}* \
  && ./configure --with-magick-plus-plus=no --with-perl=no \
  && make \
  && make install \
  && cd .. \
  && rm -r ImageMagick-${MAGICK_VERSION}* && ldconfig /usr/local/lib

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

FROM runtime
ARG CRATE_NAME
ENV MAGICK_VERSION 7.1
ARG WITH_MAGICK
RUN if [ $WITH_MAGICK = "yes" ]; then \
  apt update && apt upgrade -y && apt -y install curl build-essential clang pkg-config libjpeg-turbo-progs libpng-dev \
  && curl https://imagemagick.org/archive/ImageMagick.tar.gz | tar xz \
  && cd ImageMagick-${MAGICK_VERSION}* \
  && ./configure --with-magick-plus-plus=no --with-perl=no \
  && make \
  && make install \
  && cd .. \
  && rm -r ImageMagick-${MAGICK_VERSION}* && ldconfig /usr/local/lib;fi

RUN if [ $WITH_MAGICK = "yes" ]; then apt install -y ghostscript;fi

RUN rm -rfv /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/${CRATE_NAME} /app

ENV CRATE=${CRATE_NAME}
ENV RUST_LOG=INFO

ENTRYPOINT  "/app/${CRATE}"
