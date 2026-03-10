FROM rust:1.94 AS base

ARG TARGET=x86_64-unknown-linux-gnu

WORKDIR /base

# Rust toolchain
## Use the toolchain settings from the project
COPY rust-toolchain.toml .

## This will install the toolchain and components from rust-toolchain.toml
RUN rustup show

## Target has to be added additionally
RUN rustup target add $TARGET

FROM base AS build

ARG BUILD_TYPE="debug"
ENV CARGO_ARGS="--locked --path notis"

WORKDIR /src
COPY . .

RUN if [ "$BUILD_TYPE" = "debug" ]; then \
      cargo install $CARGO_ARGS --debug; \
    else \
      cargo install $CARGO_ARGS; \
    fi

# Move resulting binary to hardcoded path to avoid constructing path in last stage
RUN mv $CARGO_HOME/bin/notis /

FROM debian:bookworm-slim

ENV NOTIS_CONFIG_PATH="/config.json"

RUN apt-get update && \
    apt-get --yes --no-install-recommends install \
    ca-certificates && \
    apt-get --yes clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /notis /
ADD ./docker/fs /

EXPOSE 80
ENTRYPOINT [ "/notis" ]
