FROM node:22.12-alpine AS builder

WORKDIR /builder

COPY ./ui ./ui
COPY ./client/ts ./client

# Optionally specify a specific Cicada host.
ARG VITE_CICADA_HOST=""
ENV VITE_CICADA_HOST=$VITE_CICADA_HOST

# Build and link the client
RUN cd client && yarn && yarn build && yarn link
RUN cd ui && yarn link cicada_client && yarn && yarn build

# Copy just the static ui to scratch image
FROM scratch

COPY --from=builder /builder/ui/dist /ui

LABEL org.opencontainers.image.source="https://github.com/moghtech/cicada"
LABEL org.opencontainers.image.description="Cicada UI"
LABEL org.opencontainers.image.licenses="GPL-3.0"