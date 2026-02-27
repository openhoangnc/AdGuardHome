# AdGuardHome Rust Port — Multi-Architecture Dockerfile
#
# 3-stage build:
#   1. frontend — Node.js builds the React web UI
#   2. rust-builder — cross-compiles the Rust backend for the target arch
#   3. runtime — minimal Alpine image
#
# Supported platforms: linux/amd64, linux/arm64, linux/arm/v7, linux/arm/v6,
#                      linux/386, linux/ppc64le, linux/s390x

# ── Stage 1: Frontend ──────────────────────────────────────────────────────
FROM node:24-alpine AS frontend
WORKDIR /app
COPY client/ ./client/
COPY Makefile .twosky.json ./
# Install deps and build the SPA; skip if client/ doesn't exist yet.
RUN if [ -d client ]; then \
    cd client && npm ci --prefer-offline && npm run build-prod; \
  else \
    mkdir -p build; \
  fi

# ── Stage 2: Rust Backend ──────────────────────────────────────────────────
FROM --platform=$BUILDPLATFORM rust:1.93-alpine AS rust-builder
ARG TARGETPLATFORM
ARG BUILDPLATFORM

# Install musl-dev + zig (universal C cross-compiler) + cargo-zigbuild.
RUN apk add --no-cache musl-dev gcc g++ make perl git zig && \
    cargo install cargo-zigbuild

# Dynamic musl linking: omit the static libc from the binary.
# Alpine ships musl.so natively, so the binary stays small without any extra
# runtime packages.  Remove this line if you ever target a non-musl runtime.
ENV RUSTFLAGS="-C target-feature=-crt-static"

# Use /app as WORKDIR so that rust-port/ lives at /app/rust-port/ and
# #[folder = "../../../build"] in agh-web resolves to /app/build/ —
# the same path where the frontend stage places its output.
WORKDIR /app

# Map Docker platform to Rust target triple.
RUN case "$TARGETPLATFORM" in \
    "linux/amd64")   TARGET="x86_64-unknown-linux-musl" ;; \
    "linux/arm64")   TARGET="aarch64-unknown-linux-musl" ;; \
    *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
    esac && \
    rustup target add "$TARGET" && \
    echo "$TARGET" > /tmp/target

# ── Layer-cache trick: compile dependencies before copying source ──────────
# Copy only the workspace manifests and lock file first so that `cargo zigbuild`
# caches all dependencies in a dedicated layer.  Source changes won't bust
# this layer.
COPY rust-port/Cargo.toml rust-port/Cargo.lock ./rust-port/
COPY rust-port/crates/agh-core/Cargo.toml      ./rust-port/crates/agh-core/
COPY rust-port/crates/agh-filtering/Cargo.toml  ./rust-port/crates/agh-filtering/
COPY rust-port/crates/agh-dns/Cargo.toml        ./rust-port/crates/agh-dns/
COPY rust-port/crates/agh-dhcp/Cargo.toml       ./rust-port/crates/agh-dhcp/
COPY rust-port/crates/agh-stats/Cargo.toml      ./rust-port/crates/agh-stats/
COPY rust-port/crates/agh-querylog/Cargo.toml   ./rust-port/crates/agh-querylog/
COPY rust-port/crates/agh-web/Cargo.toml        ./rust-port/crates/agh-web/
COPY rust-port/crates/agh-updater/Cargo.toml    ./rust-port/crates/agh-updater/
COPY rust-port/crates/agh-main/Cargo.toml       ./rust-port/crates/agh-main/

# Create empty lib.rs / main.rs stubs so `cargo build` can fetch & compile deps.
RUN for d in agh-core agh-filtering agh-dns agh-dhcp agh-stats agh-querylog agh-web agh-updater; do \
    mkdir -p "rust-port/crates/$d/src" && echo "// stub" > "rust-port/crates/$d/src/lib.rs"; \
    done && \
    mkdir -p rust-port/crates/agh-main/src && echo "fn main(){}" > rust-port/crates/agh-main/src/main.rs

RUN TARGET=$(cat /tmp/target) && \
    cd rust-port && cargo zigbuild --release --target "$TARGET" 2>&1 | tail -5 || true

# ── Copy real source and the pre-built frontend assets ─────────────────────
COPY rust-port/crates/ ./rust-port/crates/
# Frontend assets must be present at compile time so rust-embed can embed them.
# /app/build/ matches the path that #[folder = "../../../build"] resolves to
# from CARGO_MANIFEST_DIR=/app/rust-port/crates/agh-web.
COPY --from=frontend /app/build/ ./build/

RUN TARGET=$(cat /tmp/target) && \
    cd rust-port && cargo zigbuild --release --target "$TARGET" -p agh-main && \
    cp "target/$TARGET/release/adguardhome" /adguardhome

# ── Stage 3: Runtime ───────────────────────────────────────────────────────
FROM alpine:3 AS runtime

LABEL org.opencontainers.image.source="https://github.com/AdguardTeam/AdGuardHome"
LABEL org.opencontainers.image.title="AdGuard Home (Rust port)"
LABEL org.opencontainers.image.description="Network-wide ad and tracker blocking"

RUN apk add --no-cache ca-certificates tzdata libcap && \
    mkdir -p /opt/adguardhome/conf /opt/adguardhome/work

COPY --from=rust-builder /adguardhome /opt/adguardhome/AdGuardHome

# Allow the binary to bind privileged ports (53, 67, 443, 853) without root.
RUN setcap 'cap_net_bind_service=+ep' /opt/adguardhome/AdGuardHome

VOLUME ["/opt/adguardhome/conf", "/opt/adguardhome/work"]
EXPOSE 53/tcp 53/udp 67/udp 68/udp 80/tcp 443/tcp 443/udp 3000/tcp 853/tcp 853/udp 784/udp

ENTRYPOINT ["/opt/adguardhome/AdGuardHome"]
CMD ["-c", "/opt/adguardhome/conf/AdGuardHome.yaml", \
     "-w", "/opt/adguardhome/work", \
     "--no-check-update"]

