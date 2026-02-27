#!/usr/bin/env bash
# Multi-arch Docker build using buildx — TASK-41.
#
# Usage:
#   DOCKER_REPO=myrepo/adguardhome TAG=latest bash rust-port/docker-buildx.sh
#
# Requires: docker buildx, QEMU (auto-installed via multiarch/qemu-user-static)

set -euo pipefail

DOCKER_REPO="${DOCKER_REPO:-adguardhome-rust}"
TAG="${TAG:-latest}"
PLATFORMS="${PLATFORMS:-linux/amd64,linux/arm64,linux/arm/v7,linux/arm/v6,linux/386,linux/ppc64le,linux/s390x}"
PUSH="${PUSH:-false}"

echo "=== AdGuardHome Rust port — multi-arch Docker build ==="
echo "Image: ${DOCKER_REPO}:${TAG}"
echo "Platforms: ${PLATFORMS}"
echo ""

# Register QEMU handlers for cross-compilation.
docker run --rm --privileged multiarch/qemu-user-static --reset -p yes

# Create (or reuse) a buildx builder with the docker-container driver.
docker buildx create --name agh-rust-builder --driver docker-container --use 2>/dev/null || \
  docker buildx use agh-rust-builder

docker buildx inspect --bootstrap

# Build (and optionally push) the image.
BUILD_ARGS=(
  --platform "${PLATFORMS}"
  -t "${DOCKER_REPO}:${TAG}"
  -f rust-port/Dockerfile
  .
)

if [[ "${PUSH}" == "true" ]]; then
  BUILD_ARGS+=(--push)
else
  BUILD_ARGS+=(--load)
  echo "Note: --load only works for single-platform builds."
  echo "Set PUSH=true to push a multi-arch manifest."
fi

docker buildx build "${BUILD_ARGS[@]}"

echo ""
echo "=== Build complete: ${DOCKER_REPO}:${TAG} ==="
