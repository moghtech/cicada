## Builds the Cicada Core and Periphery binaries
## for a specific architecture. Requires OpenSSL 3 or later.

## Uses chef for dependency caching to help speed up back-to-back builds.

FROM lukemathwalker/cargo-chef:latest-rust-1.95.0-bookworm AS chef
WORKDIR /builder

# Surreal's rocksdb dep requires libclang
RUN apt-get update && apt-get install -y libclang-dev

# Plan just the RECIPE to see if things have changed
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN cargo install cargo-strip

# Build JUST dependencies - cached layer
COPY --from=planner /builder/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# NOW copy again (this time into builder) and build app
COPY . .
RUN \
  cargo build --release --bin ccore && \
  cargo build --release --bin cperiphery && \
  cargo strip

# Copy just the binaries to scratch image
FROM scratch

COPY --from=builder /builder/target/release/ccore /ccore
COPY --from=builder /builder/target/release/cperiphery /cperiphery

LABEL org.opencontainers.image.source="https://github.com/moghtech/cicada"
LABEL org.opencontainers.image.description="Cicada Binaries"
LABEL org.opencontainers.image.licenses="GPL-3.0"