# TASK-42: Full CI/CD Pipeline

## Status
⬜ TODO

## Phase
Phase 11 — Docker & CI

## Dependencies
- TASK-41 ✅ (Dockerfile complete)

## Objective
Extend the CI skeleton (TASK-02) with Docker multi-arch build, security scanning, and integration test running.

---

## Checklist

- [ ] Update `.github/workflows/rust.yml` to add:

  ```yaml
  build-docker:
    name: Build Docker (single-arch test)
    runs-on: ubuntu-latest
    needs: test
    steps:
      - uses: actions/checkout@v4
      - uses: docker/setup-buildx-action@v3
      - name: Build single-arch (amd64)
        run: docker build -f rust-port/Dockerfile --platform linux/amd64 -t adguardhome-rust:ci .
      - name: Smoke test binary
        run: docker run --rm adguardhome-rust:ci --help

  publish:
    name: Build & Push Multi-Arch
    runs-on: ubuntu-latest
    needs: build-docker
    if: github.ref == 'refs/heads/main'
    steps:
      - uses: actions/checkout@v4
      - uses: docker/setup-qemu-action@v3
      - uses: docker/setup-buildx-action@v3
      - uses: docker/login-action@v3
        with:
          username: ${{ secrets.DOCKER_USERNAME }}
          password: ${{ secrets.DOCKER_PASSWORD }}
      - uses: docker/build-push-action@v6
        with:
          context: .
          file: rust-port/Dockerfile
          platforms: linux/amd64,linux/arm64,linux/arm/v7,linux/arm/v6,linux/386,linux/ppc64le,linux/s390x
          push: true
          tags: |
            ${{ secrets.DOCKER_USERNAME }}/adguardhome-rust:latest
            ${{ secrets.DOCKER_USERNAME }}/adguardhome-rust:${{ github.sha }}
  ```

- [ ] Add cargo audit job:
  ```yaml
  audit:
    name: Security Audit
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: rustsec/audit-check@v1
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
          working-directory: rust-port
  ```

- [ ] Add integration test job (runs after binary is built):
  ```yaml
  integration-test:
    name: Integration Tests
    runs-on: ubuntu-latest
    needs: check
    steps:
      - uses: actions/checkout@v4
      - run: cargo test --workspace --test api_compat
        working-directory: rust-port
        env:
          RUST_LOG: debug
  ```

- [ ] Add Dependabot config for Cargo:
  ```yaml
  # .github/dependabot.yml (append)
  - package-ecosystem: cargo
    directory: /rust-port
    schedule:
      interval: weekly
  ```

---

## Verification
Push to `main` and verify all CI jobs pass on GitHub Actions.

---

## Output Files
- Updated `.github/workflows/rust.yml`
- Updated `.github/dependabot.yml`
- Update `PROGRESS.md`: TASK-42 → ✅ DONE
