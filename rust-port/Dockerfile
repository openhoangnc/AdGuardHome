# AdGuardHome Rust Port — Multi-Architecture Dockerfile
# Supports: linux/amd64, linux/arm64, linux/arm/v7, linux/arm/v6, linux/386

FROM --platform=$BUILDPLATFORM rust:1.93 AS builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM

WORKDIR /src

# Install cross-compilation targets.
RUN case "$TARGETPLATFORM" in \
    "linux/amd64")   TARGET="x86_64-unknown-linux-musl" ;; \
    "linux/arm64")   TARGET="aarch64-unknown-linux-musl" ;; \
    "linux/arm/v7")  TARGET="armv7-unknown-linux-musleabihf" ;; \
    "linux/arm/v6")  TARGET="arm-unknown-linux-musleabihf" ;; \
    "linux/386")     TARGET="i686-unknown-linux-musl" ;; \
    *)               TARGET="x86_64-unknown-linux-musl" ;; \
    esac && \
    rustup target add "$TARGET" && \
    echo "$TARGET" > /tmp/target

# Install musl tools for static linking.
RUN apt-get update && apt-get install -y musl-tools && rm -rf /var/lib/apt/lists/*

# Copy source.
COPY rust-port/ .

# Build the binary.
RUN TARGET=$(cat /tmp/target) && \
    cargo build --release --target "$TARGET" -p agh-main && \
    cp "target/$TARGET/release/adguardhome" /adguardhome

# ── Runtime image ────────────────────────────────────────────────────────────
FROM scratch AS runtime

LABEL org.opencontainers.image.source="https://github.com/AdguardTeam/AdGuardHome"
LABEL org.opencontainers.image.title="AdGuard Home (Rust port)"
LABEL org.opencontainers.image.description="Network-wide ad and tracker blocker"

COPY --from=builder /adguardhome /usr/local/bin/adguardhome
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/

VOLUME ["/opt/adguardhome/conf", "/opt/adguardhome/work"]
EXPOSE 53/tcp 53/udp 3000/tcp 443/tcp 853/tcp 784/udp

ENTRYPOINT ["/usr/local/bin/adguardhome"]
CMD ["-c", "/opt/adguardhome/conf/AdGuardHome.yaml", \
     "-w", "/opt/adguardhome/work", \
     "--no-check-update"]
