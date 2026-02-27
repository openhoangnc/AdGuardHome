# Config YAML Schema Audit — AdGuardHome Rust Port

> **Source**: `internal/home/config.go`, `internal/dnsforward/config.go`, etc.
> **Purpose**: Authoritative mapping for TASK-06 (`AdGuardHomeConfig` serde annotations)

---

## Root Fields

| YAML key | Go type | Rust type | Default | Notes |
|---|---|---|---|---|
| `schema_version` | `int` | `u32` | `28` | Config schema version |
| `http` | `httpConfig` | `HttpConfig` | see below | HTTP server config |
| `users` | `[]webUser` | `Vec<User>` | `[]` | Admin users |
| `auth_attempts` | `uint` | `u32` | `5` | Max failed login attempts |
| `block_auth_min` | `uint` | `u32` | `15` | Block duration after failed logins (minutes) |
| `dns` | `dnsConfig` | `DnsConfig` | see below | DNS server config |
| `tls` | `tlsConfig` | `TlsConfig` | see below | TLS config |
| `filters` | `[]filterConfig` | `Vec<FilterConfig>` | `[]` | Blocklist filters |
| `whitelist_filters` | `[]filterConfig` | `Vec<FilterConfig>` | `[]` | Allowlist filters |
| `user_rules` | `[]string` | `Vec<String>` | `[]` | User-defined rules |
| `dhcp` | `dhcpdConf` | `DhcpConfig` | see below | DHCP config |
| `clients` | `clientsConf` | `ClientsConfig` | see below | Client config |
| `log` | `logSettings` | `LogConfig` | see below | Logging config |
| `os` | `osConfig` | `OsConfig` | see below | OS config |
| `statistics` | `statsConfig` | `StatisticsConfig` | see below | Stats config |
| `querylog` | `querylogConf` | `QueryLogConfig` | see below | Query log config |

---

## `http` Section

| YAML key | Go type | Rust type | Default |
|---|---|---|---|
| `address` | `string` | `String` | `"0.0.0.0:3000"` |
| `session_ttl` | `string` | `String` | `"720h"` |

## `users[]` Section

| YAML key | Go type | Rust type | Notes |
|---|---|---|---|
| `name` | `string` | `String` | Username |
| `password` | `string` | `String` | bcrypt hash |

## `dns` Section (partial — key fields)

| YAML key | Go type | Rust type | Default |
|---|---|---|---|
| `bind_hosts` | `[]netip.Addr` | `Vec<String>` | `["0.0.0.0"]` |
| `port` | `int` | `u16` | `53` |
| `upstream_dns` | `[]string` | `Vec<String>` | `["https://dns10.quad9.net/dns-query"]` |
| `bootstrap_dns` | `[]string` | `Vec<String>` | `["9.9.9.10", ...]` |
| `fallback_dns` | `[]string` | `Vec<String>` | `[]` |
| `all_servers` | `bool` | `bool` | `false` |
| `fastest_addr` | `bool` | `bool` | `false` |
| `cache_size` | `uint` | `u32` | `4194304` |
| `cache_ttl_min` | `uint` | `u32` | `0` |
| `cache_ttl_max` | `uint` | `u32` | `0` |
| `filtering_enabled` | `bool` | `bool` | `true` |
| `filters_update_interval` | `uint` | `u32` | `24` |
| `parental_enabled` | `bool` | `bool` | `false` |
| `safebrowsing_enabled` | `bool` | `bool` | `false` |
| `safe_search` | `SafeSearchConfig` | `SafeSearchConfig` | see below |
| `rewrites` | `[]rewriteEntry` | `Vec<DnsRewrite>` | `[]` |
| `aaaa_disabled` | `bool` | `bool` | `false` |
| `enable_dnssec` | `bool` | `bool` | `false` |
| `upstream_mode` | `string` | `String` | `"load_balance"` |
| `resolve_clients` | `bool` | `bool` | `true` |
| `use_private_ptr_resolvers` | `bool` | `bool` | `true` |
| `local_domain_name` | `string` | `String` | `"lan"` |

## `tls` Section

| YAML key | Go type | Rust type | Default |
|---|---|---|---|
| `enabled` | `bool` | `bool` | `false` |
| `server_name` | `string` | `String` | `""` |
| `force_https` | `bool` | `bool` | `false` |
| `port_https` | `int` | `u16` | `443` |
| `port_dns_over_tls` | `int` | `u16` | `853` |
| `port_dns_over_quic` | `int` | `u16` | `784` |
| `certificate_chain` | `string` | `String` | `""` |
| `private_key` | `string` | `String` | `""` |
| `certificate_path` | `string` | `String` | `""` |
| `private_key_path` | `string` | `String` | `""` |

## `filters[]` and `whitelist_filters[]` Section

| YAML key | Go type | Rust type | Default |
|---|---|---|---|
| `enabled` | `bool` | `bool` | `true` |
| `url` | `string` | `String` | |
| `name` | `string` | `String` | |
| `id` | `int64` | `u64` | |

## `dhcp` Section

| YAML key | Go type | Rust type | Default |
|---|---|---|---|
| `enabled` | `bool` | `bool` | `false` |
| `interface_name` | `string` | `String` | `""` |
| `local_domain_name` | `string` | `String` | `"lan"` |
| `dhcpv4.gateway_ip` | `netip.Addr` | `String` | `""` |
| `dhcpv4.subnet_mask` | `netip.Addr` | `String` | `""` |
| `dhcpv4.range_start` | `netip.Addr` | `String` | `""` |
| `dhcpv4.range_end` | `netip.Addr` | `String` | `""` |
| `dhcpv4.lease_duration` | `uint` | `u64` | `86400` |
| `dhcpv6.range_start` | `netip.Addr` | `String` | `""` |
| `dhcpv6.lease_duration` | `uint` | `u64` | `86400` |

## `log` Section

| YAML key | Go type | Rust type | Default |
|---|---|---|---|
| `enabled` | `bool` | `bool` | `true` |
| `file` | `string` | `String` | `""` (stderr) |
| `max_backups` | `int` | `i32` | `0` |
| `max_size` | `int` | `i32` | `100` (MB) |
| `max_age` | `int` | `i32` | `0` (days) |
| `compress` | `bool` | `bool` | `false` |
| `local_time` | `bool` | `bool` | `false` |
| `verbose` | `bool` | `bool` | `false` |

## `os` Section

| YAML key | Go type | Rust type | Default |
|---|---|---|---|
| `group` | `string` | `String` | `""` |
| `user` | `string` | `String` | `""` |
| `rlimit_nofile` | `uint64` | `u64` | `0` |

## `statistics` Section

| YAML key | Go type | Rust type | Default |
|---|---|---|---|
| `enabled` | `bool` | `bool` | `true` |
| `interval` | `string` | `String` | `"24h"` |
| `ignored` | `[]string` | `Vec<String>` | `[]` |

## `querylog` Section

| YAML key | Go type | Rust type | Default |
|---|---|---|---|
| `enabled` | `bool` | `bool` | `true` |
| `size_mb` | `uint32` | `u64` | `100` |
| `ignored` | `[]string` | `Vec<String>` | `[]` |
| `anonymize_client_ip` | `bool` | `bool` | `false` |
| `interval` | `string` | `String` | `"90d"` |

---

## Special Types

| Go type | Rust representation | Notes |
|---|---|---|
| `netip.Addr` | `String` | Stored as string in YAML |
| `netip.Prefix` | `String` | CIDR notation string |
| `time.Duration` | `String` | Duration strings like `"720h"`, `"24h"` |
| `[]byte` (base64) | `String` | TLS cert/key chain |
