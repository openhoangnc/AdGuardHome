use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

/// AdGuard Home — Network-level ad and tracker blocker.
#[derive(Parser, Debug)]
#[command(name = "AdGuardHome", about = "AdGuard Home — DNS ad blocker")]
pub struct Cli {
    /// Path to the config file.
    #[arg(short = 'c', long, default_value = "./AdGuardHome.yaml")]
    pub config: PathBuf,

    /// Path to the work directory.
    #[arg(short = 'w', long, default_value = "./")]
    pub work_dir: PathBuf,

    /// Host address to bind the web interface.
    #[arg(long)]
    pub host: Option<String>,

    /// Port for the web interface.
    #[arg(long)]
    pub port: Option<u16>,

    /// Skip /etc/hosts file when resolving.
    #[arg(long, default_value_t = false)]
    pub no_etc_hosts: bool,

    /// Serve frontend from disk (dev mode).
    #[arg(long, default_value_t = false)]
    pub local_frontend: bool,

    /// Disable update check.
    #[arg(long, default_value_t = false)]
    pub no_check_update: bool,

    /// Verbose (debug) logging.
    #[arg(short = 'v', long, default_value_t = false)]
    pub verbose: bool,

    /// Log file path (default: stderr).
    #[arg(long)]
    pub logfile: Option<PathBuf>,

    /// Manage system service.
    #[command(subcommand)]
    pub service: Option<ServiceCommand>,
}

/// Service management subcommand.
#[derive(Subcommand, Debug)]
pub enum ServiceCommand {
    /// Manage AdGuardHome as a system service.
    Service {
        #[command(subcommand)]
        action: ServiceAction,
    },
}

/// Service lifecycle actions.
#[derive(Subcommand, Debug)]
pub enum ServiceAction {
    Install,
    Uninstall,
    Start,
    Stop,
    Restart,
    Status,
}

/// Parse CLI arguments from the process argv.
pub fn parse() -> Cli {
    Cli::parse()
}

/// Initialise the tracing subscriber.
pub fn init_tracing(verbose: bool, _logfile: Option<&Path>) {
    let level = if verbose { "debug" } else { "info" };
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(level));
    tracing_subscriber::fmt().with_env_filter(filter).init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_flags() {
        let cli = Cli::parse_from(["adguardhome"]);
        assert_eq!(cli.config, PathBuf::from("./AdGuardHome.yaml"));
        assert_eq!(cli.work_dir, PathBuf::from("./"));
        assert!(!cli.verbose);
    }

    #[test]
    fn test_config_flag_short() {
        let cli = Cli::parse_from(["adguardhome", "-c", "/etc/adguardhome.yaml"]);
        assert_eq!(cli.config, PathBuf::from("/etc/adguardhome.yaml"));
    }

    #[test]
    fn test_service_install() {
        let cli = Cli::parse_from(["adguardhome", "service", "install"]);
        assert!(matches!(
            cli.service,
            Some(ServiceCommand::Service {
                action: ServiceAction::Install
            })
        ));
    }

    #[test]
    fn test_docker_cmd() {
        let cli = Cli::parse_from([
            "adguardhome",
            "-c",
            "/opt/adguardhome/conf/AdGuardHome.yaml",
            "-w",
            "/opt/adguardhome/work",
            "--no-check-update",
        ]);
        assert!(cli.no_check_update);
        assert_eq!(
            cli.config,
            PathBuf::from("/opt/adguardhome/conf/AdGuardHome.yaml")
        );
    }
}
