## All in one, multi stage compile + runtime Docker build for your architecture.

# Build Core
FROM rust:1.94.1-trixie AS core-builder
RUN cargo install cargo-strip

WORKDIR /builder
COPY Cargo.toml Cargo.lock ./
COPY ./lib ./lib
COPY ./client/core/rs ./client/core/rs
COPY ./client/periphery ./client/periphery
COPY ./bin/core ./bin/core

# Compile app
RUN cargo build -p cicada_core --release && \
  cargo strip

# Build UI
FROM node:22.12-alpine AS ui-builder
WORKDIR /builder
COPY ./ui ./ui
COPY ./client/core/ts ./client
RUN cd client && yarn && yarn build && yarn link
RUN cd ui && yarn link cicada_client && yarn && yarn build

# Copy binaries to distroless base
FROM gcr.io/distroless/cc

# Setup an application directory
WORKDIR /app

# Copy
COPY ./config/core.config.toml /config/.default.config.toml
COPY --from=ui-builder /builder/ui/dist /app/ui
COPY --from=core-builder /builder/target/release/core /usr/local/bin/core

# Hint at the port
EXPOSE 9220

CMD [ "core" ]

# Label for Ghcr
LABEL org.opencontainers.image.source="https://github.com/moghtech/cicada"
LABEL org.opencontainers.image.description="Cicada Core"
LABEL org.opencontainers.image.licenses="GPL-3.0"
