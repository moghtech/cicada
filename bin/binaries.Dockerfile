## Builds the Cicada Core and Periphery binaries
## for a specific architecture. Requires OpenSSL 3 or later.

FROM rust:1.94.1-bookworm AS builder

# Surreal's rocksdb dep requires libclang
RUN apt-get update && apt-get install -y libclang-dev

RUN cargo install cargo-strip

WORKDIR /builder
COPY Cargo.toml Cargo.lock ./
COPY ./lib ./lib
COPY ./client/core/rs ./client/core/rs
COPY ./client/periphery ./client/periphery
COPY ./bin/core ./bin/core
COPY ./bin/periphery ./bin/periphery

# Compile bin
RUN \
  cargo build -p cicada_core --release && \
  cargo build -p cicada_periphery --release && \
  cargo strip

# Copy just the binaries to scratch image
FROM scratch

COPY --from=builder /builder/target/release/core /core
COPY --from=builder /builder/target/release/periphery /periphery

LABEL org.opencontainers.image.source="https://github.com/moghtech/cicada"
LABEL org.opencontainers.image.description="Cicada Binaries"
LABEL org.opencontainers.image.licenses="GPL-3.0"