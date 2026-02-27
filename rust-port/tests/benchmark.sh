#!/usr/bin/env bash
# Performance benchmark script — TASK-46.
#
# Compares Go vs Rust binary performance across multiple metrics.
#
# Usage:
#   cd /path/to/AdGuardHome
#   bash rust-port/tests/benchmark.sh [--go-binary ./AdGuardHome] [--rust-binary ./rust-port/target/release/adguardhome]
#
# Requirements: cargo, curl
# Optional:     k6 (https://k6.io), dnsperf, dig
#
# Output: rust-port/docs/benchmarks.md (updated with results)

set -euo pipefail

# ── Configuration ─────────────────────────────────────────────────────────────
GO_BINARY="${GO_BINARY:-./AdGuardHome}"
RUST_BINARY="${RUST_BINARY:-./rust-port/target/release/adguardhome}"
WORK_DIR="$(mktemp -d)"
GO_PORT=13000
RUST_PORT=13001
DNS_GO_PORT=15353
DNS_RUST_PORT=15354
K6_DURATION="${K6_DURATION:-30s}"
K6_VUS="${K6_VUS:-50}"

# ── Parse args ────────────────────────────────────────────────────────────────
while [[ $# -gt 0 ]]; do
  case "$1" in
    --go-binary)   GO_BINARY="$2";   shift 2;;
    --rust-binary) RUST_BINARY="$2"; shift 2;;
    *) echo "Unknown: $1"; exit 1;;
  esac
done

# ── Helpers ───────────────────────────────────────────────────────────────────
require() { command -v "$1" &>/dev/null; }
pids=()
cleanup() {
  for pid in "${pids[@]}"; do kill "$pid" 2>/dev/null || true; done
  rm -rf "${WORK_DIR}"
}
trap cleanup EXIT

start_binary() {
  local binary="$1" config="$2" work="$3"
  "$binary" -c "$config" -w "$work" &>/dev/null &
  echo $!
}

write_config() {
  local port="$1" dns_port="$2" work="$3"
  mkdir -p "$work"
  cat > "$work/AdGuardHome.yaml" << YAML
schema_version: 28
http:
  address: 127.0.0.1:${port}
dns:
  bind_hosts:
    - 127.0.0.1
  port: ${dns_port}
  upstream_dns:
    - 8.8.8.8
  filtering_enabled: false
statistics:
  enabled: false
querylog:
  enabled: false
YAML
}

# ── Build Rust binary ─────────────────────────────────────────────────────────
echo "=== Building Rust binary ==="
cargo build --release -p agh-main 2>&1 | tail -5

# ── Start both servers ────────────────────────────────────────────────────────
GO_WORK="${WORK_DIR}/go"
RUST_WORK="${WORK_DIR}/rust"

write_config $GO_PORT $DNS_GO_PORT "$GO_WORK"
write_config $RUST_PORT $DNS_RUST_PORT "$RUST_WORK"

RUST_PID=""
if [[ -x "${RUST_BINARY}" ]]; then
  RUST_PID=$(start_binary "${RUST_BINARY}" "${RUST_WORK}/AdGuardHome.yaml" "${RUST_WORK}")
  pids+=("$RUST_PID")
  sleep 2
else
  echo "Rust binary not found: ${RUST_BINARY}"
fi

GO_PID=""
if [[ -x "${GO_BINARY}" ]]; then
  GO_PID=$(start_binary "${GO_BINARY}" "${GO_WORK}/AdGuardHome.yaml" "${GO_WORK}")
  pids+=("$GO_PID")
  sleep 2
fi

# ── Function to measure HTTP throughput via curl ──────────────────────────────
measure_http_rps() {
  local url="$1" duration="${2:-10}"
  local count=0
  local end=$((SECONDS + duration))
  while [[ $SECONDS -lt $end ]]; do
    if curl -s -o /dev/null "$url"; then
      ((count++))
    fi
  done
  echo $count
}

# ── HTTP throughput comparison ────────────────────────────────────────────────
echo ""
echo "=== HTTP API Throughput (sequential requests, 10s) ==="

RUST_HTTP_RPS="N/A"
GO_HTTP_RPS="N/A"

if [[ -n "$RUST_PID" ]]; then
  echo "Measuring Rust /control/status..."
  RUST_COUNT=$(measure_http_rps "http://127.0.0.1:${RUST_PORT}/control/status" 10)
  RUST_HTTP_RPS="${RUST_COUNT} req/10s"
fi

if [[ -n "$GO_PID" ]]; then
  echo "Measuring Go /control/status..."
  GO_COUNT=$(measure_http_rps "http://127.0.0.1:${GO_PORT}/control/status" 10)
  GO_HTTP_RPS="${GO_COUNT} req/10s"
fi

# ── DNS throughput comparison ─────────────────────────────────────────────────
echo ""
echo "=== DNS Throughput ==="
RUST_DNS_RPS="N/A"
GO_DNS_RPS="N/A"

if require dig && [[ -n "$RUST_PID" ]]; then
  RUST_DNS_COUNT=0
  for _ in $(seq 1 100); do
    dig +short "@127.0.0.1" -p ${DNS_RUST_PORT} "google.com" A &>/dev/null && ((RUST_DNS_COUNT++)) || true
  done
  RUST_DNS_RPS="${RUST_DNS_COUNT}/100 queries"
fi

if require dig && [[ -n "$GO_PID" ]]; then
  GO_DNS_COUNT=0
  for _ in $(seq 1 100); do
    dig +short "@127.0.0.1" -p ${DNS_GO_PORT} "google.com" A &>/dev/null && ((GO_DNS_COUNT++)) || true
  done
  GO_DNS_RPS="${GO_DNS_COUNT}/100 queries"
fi

# ── Memory usage ──────────────────────────────────────────────────────────────
echo ""
echo "=== Memory Usage (RSS) ==="
RUST_MEM="N/A"
GO_MEM="N/A"

if [[ -n "$RUST_PID" ]]; then
  RUST_MEM="$(ps -o rss= -p "$RUST_PID" 2>/dev/null | awk '{printf "%.1f MB", $1/1024}' || echo N/A)"
fi
if [[ -n "$GO_PID" ]]; then
  GO_MEM="$(ps -o rss= -p "$GO_PID" 2>/dev/null | awk '{printf "%.1f MB", $1/1024}' || echo N/A)"
fi

# ── Binary size ───────────────────────────────────────────────────────────────
RUST_SIZE="N/A"
GO_SIZE="N/A"
[[ -f "${RUST_BINARY}" ]] && RUST_SIZE="$(du -sh "${RUST_BINARY}" | cut -f1) (stripped musl)"
[[ -f "${GO_BINARY}" ]]   && GO_SIZE="$(du -sh "${GO_BINARY}"   | cut -f1)"

# ── k6 Load Test ─────────────────────────────────────────────────────────────
RUST_K6_P99="N/A"
GO_K6_P99="N/A"

if require k6; then
  echo ""
  echo "=== k6 Load Test (${K6_VUS} VUs, ${K6_DURATION}) ==="
  if [[ -n "$RUST_PID" ]]; then
    echo "Running k6 against Rust..."
    RUST_K6_P99=$(k6 run \
      --env BASE_URL="http://127.0.0.1:${RUST_PORT}" \
      --vus "${K6_VUS}" --duration "${K6_DURATION}" \
      rust-port/tests/k6/dns_load.js --summary-export="${WORK_DIR}/rust_k6.json" 2>/dev/null \
      && jq -r '.metrics.http_req_duration.values["p(99)"]' "${WORK_DIR}/rust_k6.json" 2>/dev/null || echo "N/A")
  fi
  if [[ -n "$GO_PID" ]]; then
    echo "Running k6 against Go..."
    GO_K6_P99=$(k6 run \
      --env BASE_URL="http://127.0.0.1:${GO_PORT}" \
      --vus "${K6_VUS}" --duration "${K6_DURATION}" \
      rust-port/tests/k6/dns_load.js --summary-export="${WORK_DIR}/go_k6.json" 2>/dev/null \
      && jq -r '.metrics.http_req_duration.values["p(99)"]' "${WORK_DIR}/go_k6.json" 2>/dev/null || echo "N/A")
  fi
fi

# ── Cargo Bench ──────────────────────────────────────────────────────────────
echo ""
echo "=== Running cargo bench ==="
cd rust-port
cargo bench --workspace 2>&1 | grep -E "time:|thrpt:|bench " || echo "(no benchmark output)"
cd ..

# ── Write results to benchmarks.md ───────────────────────────────────────────
NOW="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
cat > rust-port/docs/benchmarks.md << MD
# AdGuardHome Performance Benchmarks

> **Last updated**: ${NOW}
> **Platform**: $(uname -srm)
> **Go binary**: ${GO_BINARY}
> **Rust binary**: ${RUST_BINARY}

---

## Results

| Metric | Go | Rust | Notes |
|---|---|---|---|
| HTTP sequential RPS (\`/control/status\`) | ${GO_HTTP_RPS} | ${RUST_HTTP_RPS} | 10s sequential curl |
| HTTP p99 latency (k6 ${K6_VUS} VUs) | ${GO_K6_P99} ms | ${RUST_K6_P99} ms | k6 ${K6_DURATION} |
| DNS query success rate (100 queries) | ${GO_DNS_RPS} | ${RUST_DNS_RPS} | via dig to localhost |
| Memory (RSS, idle) | ${GO_MEM} | ${RUST_MEM} | after 10s idle |
| Binary size | ${GO_SIZE} | ${RUST_SIZE} | stripped release |

---

## Criterion Micro-benchmarks

Run with: \`cargo bench --workspace\`

| Benchmark | Result |
|---|---|
| \`check_domain_no_match\` (500k rules) | see \`criterion/\` HTML reports |
| \`check_domain_blocked\` (500k rules) | see \`criterion/\` HTML reports |
| \`engine_build_time\` (500k rules) | see \`criterion/\` HTML reports |
| DNS cache hit | see \`criterion/\` HTML reports |
| DNS cache miss | see \`criterion/\` HTML reports |

---

## Acceptance Criteria

- [ ] HTTP p99 < 50ms under 50 concurrent users
- [ ] Memory idle < 80 MB
- [ ] DNS query success rate ≥ 99/100
MD

echo ""
echo "=== Results written to rust-port/docs/benchmarks.md ==="
echo ""
echo "Summary:"
echo "  Rust HTTP: ${RUST_HTTP_RPS}   Go HTTP: ${GO_HTTP_RPS}"
echo "  Rust MEM:  ${RUST_MEM}       Go MEM:  ${GO_MEM}"
echo "  Rust SIZE: ${RUST_SIZE}      Go SIZE: ${GO_SIZE}"
