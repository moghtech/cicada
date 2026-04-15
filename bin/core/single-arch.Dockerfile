## Assumes the latest binaries for the required arch are already built (by binaries.Dockerfile).
## Sets up the necessary runtime container dependencies for Cicada Core.

ARG BINARIES_IMAGE=ghcr.io/moghtech/cicada-binaries:0

# This is required to work with COPY --from
FROM ${BINARIES_IMAGE} AS binaries

# Build UI
FROM node:22.12-alpine AS ui-builder
WORKDIR /builder
COPY ./ui ./ui
COPY ./client/ts ./client
RUN cd client && yarn && yarn build && yarn link
RUN cd ui && yarn link cicada_client && yarn && yarn build

FROM gcr.io/distroless/cc
	
# Copy
COPY ./config/core.config.toml /config/.default.config.toml
COPY --from=ui-builder /builder/ui/dist /app/ui
COPY --from=binaries /ccore /usr/local/bin/ccore

# Hint at the port
EXPOSE 9220

CMD [ "ccore" ]

# Label for Ghcr
LABEL org.opencontainers.image.source="https://github.com/moghtech/cicada"
LABEL org.opencontainers.image.description="Cicada Core"
LABEL org.opencontainers.image.licenses="GPL-3.0"
