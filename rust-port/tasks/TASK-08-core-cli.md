# TASK-08: `agh-core` — CLI Argument Parsing

## Status
⬜ TODO

## Phase
Phase 2 — `agh-core`

## Dependencies
- TASK-06 ✅ (`AdGuardHomeConfig` for default paths)

## Objective
Implement CLI argument parsing with `clap` that matches every flag accepted by the Go binary. The resulting binary must accept the same command-line interface so existing shell scripts and Docker `CMD` entries continue to work.

---

## Checklist

- [ ] Create `src/cli.rs`:

  ```rust
  use clap::{Parser, Subcommand};

  #[derive(Parser, Debug)]
  #[command(name = "AdGuardHome", about = "AdGuard Home — DNS ad blocker")]
  pub struct Cli {
      /// Path to the config file
      #[arg(short = 'c', long, default_value = "./AdGuardHome.yaml")]
      pub config: PathBuf,

      /// Path to the work directory
      #[arg(short = 'w', long, default_value = "./")]
      pub work_dir: PathBuf,

      /// Host address to bind the web interface
      #[arg(long)]
      pub host: Option<String>,

      /// Port for the web interface
      #[arg(long)]
      pub port: Option<u16>,

      /// Skip /etc/hosts file when resolving
      #[arg(long, default_value_t = false)]
      pub no_etc_hosts: bool,

      /// Serve frontend from disk (dev mode)
      #[arg(long, default_value_t = false)]
      pub local_frontend: bool,

      /// Disable update check
      #[arg(long, default_value_t = false)]
      pub no_check_update: bool,

      /// Verbose logging
      #[arg(short = 'v', long, default_value_t = false)]
      pub verbose: bool,

      /// Log file path (default: stderr)
      #[arg(long)]
      pub logfile: Option<PathBuf>,

      #[command(subcommand)]
      pub service: Option<ServiceCommand>,
  }

  #[derive(Subcommand, Debug)]
  pub enum ServiceCommand {
      /// Manage system service
      Service {
          #[command(subcommand)]
          action: ServiceAction,
      },
  }

  #[derive(Subcommand, Debug)]
  pub enum ServiceAction {
      Install,
      Uninstall,
      Start,
      Stop,
      Restart,
      Status,
  }
  ```

- [ ] Add `pub fn init_tracing(verbose: bool, logfile: Option<&Path>)` — sets `RUST_LOG` based on `verbose`, optionally writes to file via `tracing_subscriber` file appender
- [ ] Add `pub fn parse() -> Cli` as the public entrypoint

---

## Tests

```rust
#[test]
fn test_default_flags() {
    let cli = Cli::parse_from(["adguardhome"]);
    assert_eq!(cli.config, PathBuf::from("./AdGuardHome.yaml"));
}

#[test]
fn test_config_flag_short() {
    let cli = Cli::parse_from(["adguardhome", "-c", "/etc/adguardhome.yaml"]);
    assert_eq!(cli.config, PathBuf::from("/etc/adguardhome.yaml"));
}

#[test]
fn test_service_install() {
    let cli = Cli::parse_from(["adguardhome", "service", "install"]);
    assert!(matches!(cli.service, Some(ServiceCommand::Service { 
        action: ServiceAction::Install 
    })));
}

#[test]
fn test_docker_cmd() {
    // Reproduce the Docker CMD: -c /opt/.../conf/AdGuardHome.yaml -w /opt/.../work --no-check-update
    let cli = Cli::parse_from([
        "adguardhome", 
        "-c", "/opt/adguardhome/conf/AdGuardHome.yaml",
        "-w", "/opt/adguardhome/work",
        "--no-check-update"
    ]);
    assert!(cli.no_check_update);
}
```

---

## Verification
```bash
cargo test -p agh-core cli
# Also test the binary itself:
cargo run -p agh-main -- --help
```
The `--help` output should look identical to the Go binary's `--help`.

---

## Output Files
- `rust-port/crates/agh-core/src/cli.rs`
- Update `PROGRESS.md`: TASK-08 → ✅ DONE
