# TASK-03: API Contract Audit

## Status
⬜ TODO

## Phase
Phase 1 — Preparation

## Dependencies
None (can run in parallel with TASK-00).

## Objective
Produce a complete mapping of every HTTP REST endpoint from the Go source and the OpenAPI spec into a structured reference document. This document becomes the authoritative checklist for TASK-33 and TASK-34 (`agh-web` routes).

---

## Checklist

- [ ] Read `openapi/openapi.yaml` (97 KB) and extract all paths/methods
- [ ] Read `internal/home/home.go` to find all `http.HandleFunc` / router registrations
- [ ] Read `internal/dnsforward/`, `internal/filtering/`, `internal/dhcpd/`, `internal/querylog/`, `internal/stats/` handler files
- [ ] Produce `rust-port/docs/api-audit.md` with a table:
  | Method | Path | Handler File | Request Body | Response Shape | Auth Required |
  |---|---|---|---|---|---|
  (fill one row per endpoint)
- [ ] Flag any endpoints with **streaming responses** (SSE or chunked) — these need special axum handling
- [ ] Flag any endpoints using **WebSocket** upgrades
- [ ] Document the exact **cookie name and flags** used by `Set-Cookie` for auth sessions
- [ ] Document all **CORS headers** set by the Go backend
- [ ] Document how the **`X-Forwarded-For`** header is handled for trusted proxies
- [ ] Identify all endpoints that accept **multipart/form-data** vs `application/json`

---

## Key Files to Read

```
openapi/openapi.yaml
internal/home/home.go
internal/home/auth.go
internal/dnsforward/http.go
internal/filtering/http.go
internal/dhcpd/http.go
internal/querylog/http.go
internal/stats/http.go
internal/home/clientshttp.go
internal/home/config.go
```

---

## Verification
`rust-port/docs/api-audit.md` must have at least **80 rows** in the endpoint table (matching the count in `openapi.yaml`).

---

## Output Files
- `rust-port/docs/api-audit.md`
- Update `PROGRESS.md`: TASK-03 → ✅ DONE
