## Assumes the latest binaries for x86_64 and aarch64 are already built (by binaries.Dockerfile).
## Sets up the necessary runtime container dependencies for Cicada Core.
## Since theres no heavy build here, QEMU multi-arch builds are fine for this image.

ARG BINARIES_IMAGE=ghcr.io/moghtech/cicada-binaries:0
ARG UI_IMAGE=ghcr.io/moghtech/cicada-ui:0
ARG X86_64_BINARIES=${BINARIES_IMAGE}-x86_64
ARG AARCH64_BINARIES=${BINARIES_IMAGE}-aarch64

# This is required to work with COPY --from
FROM ${X86_64_BINARIES} AS x86_64
FROM ${AARCH64_BINARIES} AS aarch64
FROM ${UI_IMAGE} AS ui

# Final Image
FROM gcr.io/distroless/cc

WORKDIR /app

ARG TARGETPLATFORM

# Copy both binaries initially, but only keep appropriate one for the TARGETPLATFORM.
COPY --from=x86_64 /ccore /app/arch/linux/amd64
COPY --from=aarch64 /ccore /app/arch/linux/arm64
RUN mv /app/arch/${TARGETPLATFORM} /usr/local/bin/ccore && rm -r /app/arch

# Copy default config / static ui
COPY ./config/core.config.toml /config/.default.config.toml
COPY --from=ui /ui /app/ui

# Hint at the port
EXPOSE 9220

CMD [ "ccore" ]

# Label for Ghcr
LABEL org.opencontainers.image.source="https://github.com/moghtech/cicada"
LABEL org.opencontainers.image.description="Cicada Core"
LABEL org.opencontainers.image.licenses="GPL-3.0"
