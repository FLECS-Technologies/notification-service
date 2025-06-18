FROM --platform=linux/amd64 flecspublic.azurecr.io/flecs-build AS base

WORKDIR /base

# Rust toolchain
## Use the toolchain settings from the project
COPY rust-toolchain.toml .

## This will install the toolchain and components from rust-toolchain.toml
RUN rustup show

## Targets have to be added additionally
RUN rustup target add x86_64-unknown-linux-gnu
RUN rustup target add aarch64-unknown-linux-gnu
RUN rustup target add armv7-unknown-linux-gnueabihf

FROM base AS build

ARG TARGET=x86_64-unknown-linux-gnu
ARG BUILD_TYPE="debug"
ENV CARGO_ARGS="--target ${TARGET} --path notis"

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

RUN apt-get update && \
    apt-get --yes --no-install-recommends install \
    ca-certificates && \
    apt-get --yes clean && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /notis /
ADD ./docker/fs /

ENTRYPOINT [ "/notis" ]
