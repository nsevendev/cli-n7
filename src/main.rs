mod docker_compose;
mod resolvers;

use clap::{Parser, Subcommand};
use colored::Colorize;
use crate::docker_compose::DockerComposeCommands;

#[derive(Parser)]
#[command(name = "n7", about= "Cli de l'entreprsie nseven")]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    #[command(long_flag = "version", short_flag = 'v')]
    Version,

    #[command(name= "dc")]
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
    format!("{} - {}",
            format!("{} v{}",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
            ).green().bold(),
            format!("rust v{}", env!("CARGO_PKG_RUST_VERSION")).blue().bold()
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_version_contains_name_and_version() {
        let version = get_version();
        assert!(version.contains("n7 v"));
        assert!(version.contains("rust"));
    }
}
