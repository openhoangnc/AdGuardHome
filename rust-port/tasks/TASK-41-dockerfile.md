# TASK-41: Multi-Arch Dockerfile

## Status

⬜ TODO

## Phase

Phase 11 — Docker & CI

## Dependencies

- TASK-40 ✅ (complete binary builds successfully)

## Objective

Create the production Dockerfile with multi-arch support (`linux/amd64`, `linux/arm64`, `linux/arm/v7`, `linux/arm/v6`, `linux/386`, `linux/ppc64le`, `linux/s390x`). Port from the existing `docker/` directory approach, replacing the Go builder stage with Rust.

---

## Checklist

- [ ] Create `rust-port/Dockerfile` with 3 stages:

  **Stage 1 — Frontend** (unchanged from Go version, Node.js):

  ```dockerfile
  FROM node:24-alpine AS frontend
  WORKDIR /app
  COPY client/ ./client/
  COPY Makefile ./
  RUN cd client && npm ci && npm run build
  ```

  **Stage 2 — Rust Backend** (cross-compilation):

  ```dockerfile
  FROM --platform=$BUILDPLATFORM rust:1.93-alpine AS rust-builder
  ARG TARGETPLATFORM
  RUN apk add --no-cache musl-dev gcc g++ make perl git
  WORKDIR /build
  # Copy workspace
  COPY rust-port/Cargo.toml rust-port/Cargo.lock ./
  COPY rust-port/crates/ ./crates/
  # Map platform to Rust target
  RUN case "$TARGETPLATFORM" in \
    "linux/amd64")   TARGET="x86_64-unknown-linux-musl" ;; \
    "linux/arm64")   TARGET="aarch64-unknown-linux-musl" ;; \
    "linux/arm/v7")  TARGET="armv7-unknown-linux-musleabihf" ;; \
    "linux/arm/v6")  TARGET="arm-unknown-linux-musleabihf" ;; \
    "linux/386")     TARGET="i686-unknown-linux-musl" ;; \
    "linux/ppc64le") TARGET="powerpc64le-unknown-linux-gnu" ;; \
    "linux/s390x")   TARGET="s390x-unknown-linux-gnu" ;; \
    *) echo "Unsupported: $TARGETPLATFORM" && exit 1 ;; \
  esac && \
  rustup target add $TARGET && \
  cargo build --release --target $TARGET --bin adguardhome && \
  cp target/$TARGET/release/adguardhome /adguardhome
  ```

  **Stage 3 — Final minimal image**:

  ```dockerfile
  FROM alpine:3.20
  RUN apk add --no-cache ca-certificates tzdata libcap && \
      setcap 'cap_net_bind_service=+ep' /opt/adguardhome/AdGuardHome
  COPY --from=rust-builder /adguardhome /opt/adguardhome/AdGuardHome
  COPY --from=frontend /app/build /opt/adguardhome/build
  VOLUME ["/opt/adguardhome/conf", "/opt/adguardhome/work"]
  EXPOSE 53/tcp 53/udp 67/udp 68/udp 80/tcp 443/tcp 443/udp 3000/tcp 853/tcp 853/udp 5443/tcp 5443/udp 6060/tcp
  ENTRYPOINT ["/opt/adguardhome/AdGuardHome"]
  CMD ["-c", "/opt/adguardhome/conf/AdGuardHome.yaml", "-w", "/opt/adguardhome/work", "--no-check-update"]
  ```

- [ ] Create `rust-port/docker-buildx.sh`:

  ```bash
  #!/usr/bin/env bash
  set -euo pipefail
  docker buildx create --name agh-builder --driver docker-container --use 2>/dev/null || true
  docker run --rm --privileged multiarch/qemu-user-static --reset -p yes
  docker buildx build \
    --platform linux/amd64,linux/arm64,linux/arm/v7,linux/arm/v6,linux/386,linux/ppc64le,linux/s390x \
    -t "${DOCKER_REPO:-adguardhome-rust}:${TAG:-latest}" \
    --push \
    -f rust-port/Dockerfile \
    .
  ```

- [ ] Create `.dockerignore` excluding `target/`, `client/node_modules/`, `.git/`
- [ ] Layer caching optimization: copy `Cargo.toml`/`Cargo.lock` first, build empty project to cache dependencies, then copy source

---

## Verification

```bash
# Single-arch test (fast):
docker build -f rust-port/Dockerfile -t adguardhome-rust:test .
docker run --rm adguardhome-rust:test --help
# Multi-arch build (slow):
bash rust-port/docker-buildx.sh
```

---

## Output Files

- `rust-port/Dockerfile`
- `rust-port/docker-buildx.sh`
- `.dockerignore` (update or create)
- Update `PROGRESS.md`: TASK-41 → ✅ DONE
