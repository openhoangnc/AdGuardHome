# TASK-02: CI/CD Skeleton (GitHub Actions)

## Status
⬜ TODO

## Phase
Phase 0 — Scaffold

## Dependencies
- TASK-01 ✅ (all crate skeletons must exist)

## Objective
Create the GitHub Actions CI workflow that will validate every subsequent task's output. The CI must run on every push and pull request.

---

## Checklist

- [ ] Create `.github/workflows/rust.yml`:
  ```yaml
  name: Rust Build & Test

  on:
    push:
      paths:
        - 'rust-port/**'
        - '.github/workflows/rust.yml'
    pull_request:
      paths:
        - 'rust-port/**'

  defaults:
    run:
      working-directory: rust-port

  jobs:
    check:
      name: Check & Clippy
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v4
        - uses: dtolnay/rust-toolchain@stable
          with:
            components: clippy
        - uses: Swatinem/rust-cache@v2
          with:
            workspaces: rust-port
        - run: cargo check --workspace
        - run: cargo clippy --workspace -- -D warnings

    test:
      name: Test
      runs-on: ubuntu-latest
      needs: check
      steps:
        - uses: actions/checkout@v4
        - uses: dtolnay/rust-toolchain@stable
        - uses: Swatinem/rust-cache@v2
          with:
            workspaces: rust-port
        - run: cargo test --workspace

    fmt:
      name: Format Check
      runs-on: ubuntu-latest
      steps:
        - uses: actions/checkout@v4
        - uses: dtolnay/rust-toolchain@stable
          with:
            components: rustfmt
        - run: cargo fmt --all -- --check
  ```

- [ ] Add `rust-port/rustfmt.toml`:
  ```toml
  edition = "2024"
  max_width = 100
  use_small_heuristics = "Default"
  ```

- [ ] Add `rust-port/clippy.toml`:
  ```toml
  msrv = "1.93"
  ```

- [ ] Verify locally:
  ```bash
  cd rust-port
  cargo fmt --all -- --check
  cargo clippy --workspace -- -D warnings
  ```

---

## Verification
Push the branch and confirm GitHub Actions shows green on all 3 jobs (`check`, `test`, `fmt`).

---

## Output Files
- `.github/workflows/rust.yml`
- `rust-port/rustfmt.toml`
- `rust-port/clippy.toml`
- Update `PROGRESS.md`: TASK-02 → ✅ DONE
