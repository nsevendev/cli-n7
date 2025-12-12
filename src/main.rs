mod docker_compose;
mod resolvers;

use crate::docker_compose::DockerComposeCommands;
use clap::builder::styling::{AnsiColor, Styles};
use clap::{Parser, Subcommand};
use colored::Colorize;

fn styles() -> Styles {
    Styles::styled()
        .context(AnsiColor::Yellow.on_default())
        .header(AnsiColor::Magenta.on_default())
        .usage(AnsiColor::Magenta.on_default())
        .literal(AnsiColor::Green.on_default())
        .placeholder(AnsiColor::Yellow.on_default())
        .error(AnsiColor::Red.on_default())
        .valid(AnsiColor::Green.on_default())
        .invalid(AnsiColor::Yellow.on_default())
}

#[derive(Parser)]
#[command(name = "n7", about = "\x1b[33mCli nseven\x1b[0m", styles = styles())]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(
        long_flag = "version",
        short_flag = 'v',
        about = "\x1b[33mPrint Version info\x1b[0m"
    )]
    Version,

    #[command(name = "dc", about = "\x1b[33mExecute docker compose\x1b[0m")]
    DockerCompose {
        #[command(subcommand)]
        action: DockerComposeCommands,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Version => {
            println!("{}", get_version());
        }
        Commands::DockerCompose { action } => {
            action.execute().unwrap_or_else(|e| {
                eprintln!("{}", format!("Error: {}", e).red());
                std::process::exit(1);
            });
        }
    }
}

fn get_version() -> String {
    format!(
        "{} - {}",
        format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
            .green()
            .bold(),
        format!("rust v{}", env!("CARGO_PKG_RUST_VERSION"))
            .blue()
            .bold()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version_contains_name_and_version() {
        let _lock = n7::test_utils::lock_test();
        let version = get_version();
        assert!(version.contains("n7 v"));
        assert!(version.contains("rust"));
    }
}
