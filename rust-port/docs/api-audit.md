# API Contract Audit — AdGuardHome Rust Port

> **Generated**: 2026-02-26  
> **Source**: `openapi/openapi.yaml` + `internal/home/` Go source files  
> **Total endpoints**: 81 (all under `/control` prefix)

---

## Endpoint Table

| Method | Path (full) | Module | Auth Required | Notes |
|---|---|---|---|---|
| GET | `/control/status` | global | Yes | Server status overview |
| GET | `/control/dns_info` | global | Yes | DNS settings read |
| POST | `/control/dns_config` | global | Yes | DNS settings write |
| POST | `/control/protection` | global | Yes | Enable/disable protection |
| POST | `/control/cache_clear` | global | Yes | Clear DNS cache |
| POST | `/control/test_upstream_dns` | global | Yes | Test upstream servers |
| GET | `/control/version.json` | global | No | Version check |
| POST | `/control/update` | global | Yes | Trigger self-update |
| GET | `/control/querylog` | querylog | Yes | Read query log (pagination) |
| GET | `/control/querylog_info` | querylog | Yes | Query log config read |
| POST | `/control/querylog_config` | querylog | Yes | Query log config write (legacy) |
| POST | `/control/querylog_clear` | querylog | Yes | Clear query log |
| GET | `/control/querylog/config` | querylog | Yes | Query log config read (new) |
| PUT | `/control/querylog/config/update` | querylog | Yes | Query log config write (new) |
| GET | `/control/stats` | stats | Yes | Statistics read |
| POST | `/control/stats_reset` | stats | Yes | Reset statistics |
| GET | `/control/stats_info` | stats | Yes | Stats config read (legacy) |
| POST | `/control/stats_config` | stats | Yes | Stats config write (legacy) |
| GET | `/control/stats/config` | stats | Yes | Stats config read (new) |
| PUT | `/control/stats/config/update` | stats | Yes | Stats config write (new) |
| GET | `/control/tls/status` | tls | Yes | TLS config read |
| POST | `/control/tls/configure` | tls | Yes | TLS config write |
| POST | `/control/tls/validate` | tls | Yes | Validate TLS config |
| GET | `/control/dhcp/status` | dhcp | Yes | DHCP server status |
| GET | `/control/dhcp/interfaces` | dhcp | Yes | List network interfaces |
| POST | `/control/dhcp/set_config` | dhcp | Yes | DHCP config write |
| POST | `/control/dhcp/find_active_dhcp` | dhcp | Yes | Find active DHCP servers |
| POST | `/control/dhcp/add_static_lease` | dhcp | Yes | Add static DHCP lease |
| POST | `/control/dhcp/remove_static_lease` | dhcp | Yes | Remove static lease |
| POST | `/control/dhcp/update_static_lease` | dhcp | Yes | Update static lease |
| POST | `/control/dhcp/reset` | dhcp | Yes | Reset DHCP config |
| POST | `/control/dhcp/reset_leases` | dhcp | Yes | Delete all leases |
| GET | `/control/filtering/status` | filtering | Yes | Filtering status |
| POST | `/control/filtering/config` | filtering | Yes | Filtering config write |
| POST | `/control/filtering/add_url` | filtering | Yes | Add filter list |
| POST | `/control/filtering/remove_url` | filtering | Yes | Remove filter list |
| POST | `/control/filtering/set_url` | filtering | Yes | Update filter list |
| POST | `/control/filtering/refresh` | filtering | Yes | Force refresh filter lists |
| POST | `/control/filtering/set_rules` | filtering | Yes | Set user rules |
| GET | `/control/filtering/check_host` | filtering | Yes | Check if host is blocked |
| POST | `/control/safebrowsing/enable` | safebrowsing | Yes | Enable safe browsing |
| POST | `/control/safebrowsing/disable` | safebrowsing | Yes | Disable safe browsing |
| GET | `/control/safebrowsing/status` | safebrowsing | Yes | Safe browsing status |
| POST | `/control/parental/enable` | parental | Yes | Enable parental control |
| POST | `/control/parental/disable` | parental | Yes | Disable parental control |
| GET | `/control/parental/status` | parental | Yes | Parental control status |
| POST | `/control/safesearch/enable` | safesearch | Yes | Enable safe search |
| POST | `/control/safesearch/disable` | safesearch | Yes | Disable safe search |
| GET | `/control/safesearch/settings` | safesearch | Yes | Safe search settings |
| GET | `/control/safesearch/status` | safesearch | Yes | Safe search status |
| GET | `/control/clients` | clients | Yes | List all clients |
| POST | `/control/clients/add` | clients | Yes | Add persistent client |
| POST | `/control/clients/delete` | clients | Yes | Delete persistent client |
| POST | `/control/clients/update` | clients | Yes | Update persistent client |
| GET | `/control/clients/find` | clients | Yes | Find client by IP |
| POST | `/control/clients/search` | clients | Yes | Search clients |
| GET | `/control/access/list` | access | Yes | Access control list |
| POST | `/control/access/set` | access | Yes | Set access control |
| GET | `/control/blocked_services/services` | blocked_services | Yes | List all services |
| GET | `/control/blocked_services/all` | blocked_services | Yes | All blocked services |
| GET | `/control/blocked_services/list` | blocked_services | Yes | Current blocked services |
| POST | `/control/blocked_services/set` | blocked_services | Yes | Set blocked services |
| GET | `/control/blocked_services/get` | blocked_services | Yes | Get blocked services |
| PUT | `/control/blocked_services/update` | blocked_services | Yes | Update blocked services |
| GET | `/control/rewrite/list` | rewrite | Yes | List DNS rewrites |
| POST | `/control/rewrite/add` | rewrite | Yes | Add DNS rewrite |
| POST | `/control/rewrite/delete` | rewrite | Yes | Delete DNS rewrite |
| GET | `/control/rewrite/settings` | rewrite | Yes | Rewrite settings |
| PUT | `/control/rewrite/settings/update` | rewrite | Yes | Update rewrite settings |
| POST | `/control/rewrite/update` | rewrite | Yes | Update a rewrite rule |
| POST | `/control/i18n/change_language` | i18n | Yes | Change UI language |
| GET | `/control/i18n/current_language` | i18n | Yes | Get current language |
| GET | `/control/install/get_addresses` | install | **No** | Get bind addresses for wizard |
| POST | `/control/install/check_config` | install | **No** | Check setup config |
| POST | `/control/install/configure` | install | **No** | Complete setup wizard |
| POST | `/control/login` | auth | **No** | Login — sets `agh_session` cookie |
| GET | `/control/logout` | auth | Yes | Logout — clears `agh_session` cookie |
| PUT | `/control/profile/update` | profile | Yes | Update user profile |
| GET | `/control/profile` | profile | Yes | Get user profile |
| GET | `/control/apple/doh.mobileconfig` | apple | No | Apple DoH profile download |
| GET | `/control/apple/dot.mobileconfig` | apple | No | Apple DoT profile download |

---

## Streaming Endpoints

| Path | Type | Notes |
|---|---|---|
| `GET /control/querylog` | Paginated JSON | Uses cursor-based pagination; `older_than` query param |

## Authentication Details

- **Cookie name**: `agh_session`
- **Cookie flags**: `HttpOnly; Path=/` (plus `Secure` when TLS is enabled)
- **Session TTL**: Configurable, default `720h` (30 days)
- **Rate limiting**: After N failed logins, block for M minutes (configurable)

## CORS Headers

The Go backend sets permissive CORS for `/control/` endpoints when `trusted_proxies` is configured. Exact headers:
- `Access-Control-Allow-Origin: *` (configurable)
- `Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS`

## Multipart Endpoints

None — all endpoints use `application/json` or no body.
