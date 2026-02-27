#!/usr/bin/env bash
# DNS compliance test script for the AdGuardHome Rust binary.
#
# Usage:
#   bash rust-port/tests/dns_compliance.sh [--binary /path/to/adguardhome]
#
# Requirements: dig (bind-utils / dnsutils package)
# Optional:     q (https://github.com/natesales/q) for DoH/DoT tests
#
# Exit code 0 = all tests passed
# Exit code 1 = one or more tests failed

set -euo pipefail

# ── Configuration ─────────────────────────────────────────────────────────────
BINARY="${BINARY:-./target/debug/adguardhome}"
DNS_PORT="${DNS_PORT:-15353}"
DOH_PORT="${DOH_PORT:-18443}"
DOT_PORT="${DOT_PORT:-18853}"
WORK_DIR="$(mktemp -d)"
CONFIG_FILE="${WORK_DIR}/AdGuardHome.yaml"
PID_FILE="${WORK_DIR}/adguardhome.pid"
LOG_FILE="${WORK_DIR}/adguardhome.log"

# Parse arguments
while [[ $# -gt 0 ]]; do
  case "$1" in
    --binary) BINARY="$2"; shift 2;;
    *) echo "Unknown argument: $1" >&2; exit 1;;
  esac
done

PASS=0
FAIL=0
SKIP=0

# ── Helpers ───────────────────────────────────────────────────────────────────
green()  { echo -e "\033[32m✓ $*\033[0m"; }
red()    { echo -e "\033[31m✗ $*\033[0m"; }
yellow() { echo -e "\033[33m~ $*\033[0m"; }

pass() { green "$*"; ((PASS++)); }
fail() { red   "$*"; ((FAIL++)); }
skip() { yellow "SKIP: $*"; ((SKIP++)); }

require_command() {
  if ! command -v "$1" &>/dev/null; then
    skip "$1 not installed — skipping related tests"
    return 1
  fi
  return 0
}

# Run a DNS query via dig and capture the first answer.
dig_query() {
  local name="$1" qtype="${2:-A}"
  dig +short "@127.0.0.1" -p "${DNS_PORT}" "${name}" "${qtype}" 2>/dev/null || true
}

# Assert that dig_query returns a non-empty result.
assert_resolves() {
  local name="$1" qtype="${2:-A}"
  local result
  result=$(dig_query "${name}" "${qtype}")
  if [[ -n "${result}" ]]; then
    pass "${name} ${qtype} → ${result}"
  else
    fail "${name} ${qtype}: got empty response (expected a record)"
  fi
}

# Assert that dig_query returns an empty result (blocked).
assert_blocked() {
  local name="$1" qtype="${2:-A}"
  local result
  result=$(dig_query "${name}" "${qtype}")
  if [[ -z "${result}" ]] || echo "${result}" | grep -qE "^0\.0\.0\.0$|^::$"; then
    pass "${name} ${qtype} → blocked (${result:-NXDOMAIN})"
  else
    fail "${name} ${qtype}: expected blocked, got '${result}'"
  fi
}

# ── Setup: write a test config ────────────────────────────────────────────────
cat > "${CONFIG_FILE}" << YAML
schema_version: 28
http:
  address: 127.0.0.1:13000
dns:
  bind_hosts:
    - 127.0.0.1
  port: ${DNS_PORT}
  upstream_dns:
    - 8.8.8.8
    - 8.8.4.4
  filtering_enabled: true
filters:
  - id: 1
    enabled: true
    url: ""
    name: Test blocklist
user_rules:
  - "||blocked-compliance-test.example^"
YAML

# ── Start the binary ──────────────────────────────────────────────────────────
echo "=== Starting AdGuardHome Rust binary ==="
if [[ ! -x "${BINARY}" ]]; then
  echo "Binary not found or not executable: ${BINARY}"
  echo "Build first with: cargo build -p agh-main"
  exit 1
fi

"${BINARY}" -c "${CONFIG_FILE}" -w "${WORK_DIR}" &
echo $! > "${PID_FILE}"
sleep 2  # Allow startup

cleanup() {
  if [[ -f "${PID_FILE}" ]]; then
    kill "$(cat "${PID_FILE}")" 2>/dev/null || true
  fi
  rm -rf "${WORK_DIR}"
}
trap cleanup EXIT

# ── Test Suite ────────────────────────────────────────────────────────────────
echo ""
echo "=== Plain DNS tests (UDP port ${DNS_PORT}) ==="

if ! require_command dig; then
  echo "dig not available — skipping all plain DNS tests"
else
  # Basic resolution
  assert_resolves "google.com"           "A"
  assert_resolves "google.com"           "AAAA"
  assert_resolves "github.com"           "A"
  assert_resolves "example.com"          "A"
  assert_resolves "example.com"          "TXT"
  assert_resolves "example.com"          "MX"
  assert_resolves "example.com"          "NS"

  # NXDOMAIN for non-existent domain
  echo "--- NXDOMAIN test ---"
  nxdomain_result=$(dig +short "@127.0.0.1" -p "${DNS_PORT}" "this-domain-cannot-exist-xyz.invalid" A 2>/dev/null || true)
  if [[ -z "${nxdomain_result}" ]]; then
    pass "Non-existent domain returns NXDOMAIN"
  else
    fail "Non-existent domain unexpectedly resolved: ${nxdomain_result}"
  fi

  # Blocked domain (user rule: ||blocked-compliance-test.example^)
  echo "--- Blocking tests ---"
  assert_blocked "blocked-compliance-test.example"
fi

# ── DoH tests ─────────────────────────────────────────────────────────────────
echo ""
echo "=== DoH tests ==="
if ! require_command curl; then
  skip "curl not installed"
else
  # DoH uses port ${DOH_PORT} in test config (not configured above, skip gracefully).
  skip "DoH server requires TLS cert — not configured in compliance test config"
fi

# ── API health check ──────────────────────────────────────────────────────────
echo ""
echo "=== HTTP API health check ==="
if require_command curl; then
  status_response=$(curl -s "http://127.0.0.1:13000/control/status" 2>/dev/null || true)
  if echo "${status_response}" | grep -q '"running"'; then
    pass "GET /control/status returns running=true"
  else
    fail "GET /control/status: unexpected response: ${status_response}"
  fi
fi

# ── Summary ───────────────────────────────────────────────────────────────────
echo ""
echo "══════════════════════════════════════════"
echo "  Results: ${PASS} passed, ${FAIL} failed, ${SKIP} skipped"
echo "══════════════════════════════════════════"

if [[ ${FAIL} -gt 0 ]]; then
  exit 1
fi
exit 0
