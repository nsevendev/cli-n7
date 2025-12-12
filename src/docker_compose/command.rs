use super::service::up::UpService;
use crate::docker_compose::service::down::DownService;
use crate::docker_compose::service::shell::ShellService;
use clap::Subcommand;
use std::process::Command;

#[derive(Subcommand)]
pub enum DockerComposeCommands {
    #[command(name = "u", about = "\x1b[33mCommand 'up' with options\x1b[0m")]
    Up {
        #[arg(
            short,
            long,
            help = "\x1b[33mBuild images before starting containers\x1b[0m"
        )]
        build: bool,

        #[arg(short, long, help = "\x1b[33mDo not run in detached mode\x1b[0m")]
        no_detach: bool,

        #[arg(
            short = 'e',
            long,
            help = "\x1b[33mPath to the environment file\x1b[0m"
        )]
        env_file: Option<String>,

        #[arg(short = 'f', long, help = "\x1b[33mPath to the compose file\x1b[0m")]
        compose_file: Option<String>,
    },

    #[command(name = "d", about = "\x1b[33mCommand 'down' with options\x1b[0m")]
    Down {
        #[arg(
            short = 'v',
            long = "rmv",
            help = "\x1b[33mDelete all volume of service in compose file\x1b[0m"
        )]
        rmvolumes: bool,

        #[arg(
            short = 'o',
            long = "rmo",
            help = "\x1b[33mDelete container orphans not in compose file\x1b[0m"
        )]
        rmorphans: bool,
    },

    #[command(
        name = "s",
        about = "\x1b[33mConnect to service shell (interactive mode)\x1b[0m"
    )]
    Shell {
        #[arg(help = "\x1b[33mService name to connect to\x1b[0m")]
        service: String,

        #[arg(
            short = 's',
            long,
            help = "\x1b[33mShell to use (default: bash)\x1b[0m"
        )]
        shell: Option<String>,
    },
}

impl DockerComposeCommands {
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        match self {
            DockerComposeCommands::Up {
                build,
                no_detach,
                env_file,
                compose_file,
            } => {
                let args =
                    UpService::up(*build, *no_detach, env_file.clone(), compose_file.clone());

                println!("Command execute : {}", args.join(" "));

                // mode dry run (pour les tests), on ne lance pas la commande
                if std::env::var("N7_DRY_RUN").is_ok() {
                    return Ok(());
                }

                // Exécute la commande
                let status = Command::new(&args[0]).args(&args[1..]).status()?;

                if status.success() {
                    Ok(())
                } else {
                    Err(format!("Command failed with exit code: {:?}", status.code()).into())
                }
            }

            DockerComposeCommands::Down {
                rmvolumes,
                rmorphans,
            } => {
                let args = DownService::down(*rmvolumes, *rmorphans);
                println!("Command execute : {}", args.join(" "));

                if std::env::var("N7_DRY_RUN").is_ok() {
                    return Ok(());
                }

                let status = Command::new(&args[0]).args(&args[1..]).status()?;

                if status.success() {
                    Ok(())
                } else {
                    Err(format!("Command failed with exit code: {:?}", status.code()).into())
                }
            }

            DockerComposeCommands::Shell { service, shell } => {
                let mut args = ShellService::shell(service.clone(), shell.clone());

                args.insert(3, "-it".to_string());

                println!("Command execute : {}", args.join(" "));

                if std::env::var("N7_DRY_RUN").is_ok() {
                    return Ok(());
                }

                let status = Command::new(&args[0]).args(&args[1..]).status()?;

                if status.success() {
                    Ok(())
                } else {
                    Err(format!("Command failed with exit code: {:?}", status.code()).into())
                }
            }
        }
    }
}
