/**
 * k6 load test for AdGuardHome Rust binary HTTP API — TASK-46.
 *
 * Usage:
 *   k6 run --env BASE_URL=http://localhost:3000 rust-port/tests/k6/dns_load.js
 *
 * Install k6: https://k6.io/docs/getting-started/installation/
 */
import http from 'k6/http';
import { check, sleep } from 'k6';
import { Rate, Trend } from 'k6/metrics';

// ── Options ────────────────────────────────────────────────────────────────
export const options = {
  stages: [
    { duration: '10s', target: 10 },   // ramp-up
    { duration: '30s', target: 100 },  // steady load
    { duration: '10s', target: 0 },    // ramp-down
  ],
  thresholds: {
    http_req_failed: ['rate<0.01'],              // <1% errors
    http_req_duration: ['p(99)<50'],             // 99th percentile <50ms
    'status_ok': ['rate>0.99'],                  // >99% OK responses
  },
};

// ── Custom metrics ──────────────────────────────────────────────────────────
const statusOkRate = new Rate('status_ok');
const dnsInfoDuration = new Trend('dns_info_duration');
const filteringStatusDuration = new Trend('filtering_status_duration');

// ── Default function ────────────────────────────────────────────────────────
const BASE_URL = __ENV.BASE_URL || 'http://localhost:3000';

export default function () {
  // GET /control/status
  const statusResp = http.get(`${BASE_URL}/control/status`);
  const statusOk = check(statusResp, {
    'status 200': (r) => r.status === 200,
    'has running field': (r) => {
      try {
        return JSON.parse(r.body).running !== undefined;
      } catch {
        return false;
      }
    },
  });
  statusOkRate.add(statusOk);

  // GET /control/dns_info
  const dnsInfoResp = http.get(`${BASE_URL}/control/dns_info`);
  dnsInfoDuration.add(dnsInfoResp.timings.duration);
  check(dnsInfoResp, {
    'dns_info status 200': (r) => r.status === 200,
  });

  // GET /control/filtering/status
  const filterResp = http.get(`${BASE_URL}/control/filtering/status`);
  filteringStatusDuration.add(filterResp.timings.duration);
  check(filterResp, {
    'filtering_status status 200': (r) => r.status === 200,
  });

  // GET /control/version.json
  const versionResp = http.get(`${BASE_URL}/control/version.json`);
  check(versionResp, {
    'version status 200': (r) => r.status === 200,
  });

  sleep(0.01);
}

export function handleSummary(data) {
  return {
    'stdout': textSummary(data, { indent: ' ', enableColors: true }),
  };
}

function textSummary(data, opts) {
  const indent = opts.indent || '';
  const lines = [];
  lines.push(`${indent}=== k6 Load Test Summary ===`);
  lines.push(`${indent}VUs max: ${data.options.stages[1].target}`);
  const m = data.metrics;
  if (m.http_req_duration) {
    lines.push(`${indent}HTTP p50: ${m.http_req_duration.values.p(50).toFixed(1)}ms`);
    lines.push(`${indent}HTTP p99: ${m.http_req_duration.values.p(99).toFixed(1)}ms`);
  }
  if (m.http_req_failed) {
    lines.push(`${indent}Error rate: ${(m.http_req_failed.values.rate * 100).toFixed(2)}%`);
  }
  return lines.join('\n');
}
