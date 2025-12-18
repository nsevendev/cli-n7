use super::service::up::UpService;
use crate::docker_compose::service::cargo_exec::CargoExecService;
use crate::docker_compose::service::down::DownService;
use crate::docker_compose::service::exec::ExecService;
use crate::docker_compose::service::logs::LogsService;
use crate::docker_compose::service::shell::ShellService;
use crate::resolvers::ComposeServices;
use clap::Subcommand;
use colored::Colorize;
use std::process::Command;

#[derive(Subcommand)]
pub enum DockerComposeCommands {
    #[command(name = "u", about = "Command 'up' with options")]
    Up {
        #[arg(short, long, help = "Build images before starting containers")]
        build: bool,

        #[arg(short, long, help = "Do not run in detached mode")]
        no_detach: bool,

        #[arg(short = 'e', long, help = "Path to the environment file")]
        env_file: Option<String>,

        #[arg(short = 'f', long, help = "Path to the compose file")]
        compose_file: Option<String>,
    },

    #[command(name = "d", about = "Command 'down' with options")]
    Down {
        #[arg(
            short = 'v',
            long = "rmv",
            help = "Delete all volume of service in compose file"
        )]
        rmvolumes: bool,

        #[arg(
            short = 'o',
            long = "rmo",
            help = "Delete container orphans not in compose file"
        )]
        rmorphans: bool,
    },

    #[command(name = "s", about = "Connect to service shell (interactive mode)")]
    Shell {
        #[arg(help = "Service name to connect to")]
        service: String,

        #[arg(short = 's', long, help = "Shell to use (default: bash)")]
        shell: Option<String>,
    },

    #[command(name = "l", about = "Show logs from services")]
    Logs {
        #[arg(help = "Service name (optional, default: all services)")]
        service: Option<String>,

        #[arg(
            short = 'n',
            long = "no-follow",
            help = "Disable follow mode (default: follow enabled)"
        )]
        no_follow: bool,
    },

    #[command(name = "ex", about = "Execute custom command in service container")]
    Exec {
        #[arg(help = "Docker service name (optional, lists available services if not provided)")]
        service: Option<String>,

        #[arg(short = 'd', long = "detach", help = "Run command in detached mode")]
        detach: bool,

        #[arg(last = true, help = "Command and arguments to execute")]
        args: Option<Vec<String>>,
    },

    #[command(name = "c", about = "Execute custom cargo command")]
    Cargo {
        #[arg(help = "Docker service name (optional, lists available services if not provided)")]
        service: Option<String>,

        #[arg(last = true, help = "Cargo command and arguments")]
        args: Option<Vec<String>>,
    },

    #[command(name = "ct", about = "Run cargo test")]
    CargoTest {
        #[arg(help = "Docker service name (optional, lists available services if not provided)")]
        service: Option<String>,

        #[arg(last = true, help = "Additional arguments for cargo test")]
        args: Option<Vec<String>>,
    },

    #[command(name = "cf", about = "Run cargo fmt")]
    CargoFmt {
        #[arg(help = "Docker service name (optional, lists available services if not provided)")]
        service: Option<String>,

        #[arg(last = true, help = "Additional arguments for cargo fmt")]
        args: Option<Vec<String>>,
    },

    #[command(name = "cc", about = "Run cargo clippy")]
    CargoClippy {
        #[arg(help = "Docker service name (optional, lists available services if not provided)")]
        service: Option<String>,

        #[arg(last = true, help = "Additional arguments for cargo clippy")]
        args: Option<Vec<String>>,
    },

    #[command(name = "ccov", about = "Run cargo llvm-cov")]
    CargoLlvmCov {
        #[arg(help = "Docker service name (optional, lists available services if not provided)")]
        service: Option<String>,

        #[arg(last = true, help = "Additional arguments for cargo llvm-cov")]
        args: Option<Vec<String>>,
    },

    #[command(name = "cck", about = "Run fmt, clippy, and test in sequence")]
    Rcheck {
        #[arg(help = "Docker service name (optional, lists available services if not provided)")]
        service: Option<String>,
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

            DockerComposeCommands::Logs { service, no_follow } => {
                let follow = !no_follow;
                let args = LogsService::logs(service.clone(), follow);

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

            DockerComposeCommands::Exec {
                service,
                detach,
                args,
            } => {
                if service.is_none() || args.is_none() {
                    ComposeServices::display_available_services();
                    return Ok(());
                }

                let cmd_args =
                    ExecService::exec(service.clone().unwrap(), *detach, args.clone().unwrap());

                println!("Command execute : {}", cmd_args.join(" "));

                if std::env::var("N7_DRY_RUN").is_ok() {
                    return Ok(());
                }

                let status = Command::new(&cmd_args[0]).args(&cmd_args[1..]).status()?;

                if status.success() {
                    Ok(())
                } else {
                    Err(format!("Command failed with exit code: {:?}", status.code()).into())
                }
            }

            DockerComposeCommands::Cargo { service, args } => {
                if service.is_none() || args.is_none() {
                    ComposeServices::display_available_services();
                    return Ok(());
                }

                let cmd_args =
                    CargoExecService::cargo(service.clone().unwrap(), args.clone().unwrap());

                println!("Command execute : {}", cmd_args.join(" "));

                if std::env::var("N7_DRY_RUN").is_ok() {
                    return Ok(());
                }

                let status = Command::new(&cmd_args[0]).args(&cmd_args[1..]).status()?;

                if status.success() {
                    Ok(())
                } else {
                    Err(format!("Command failed with exit code: {:?}", status.code()).into())
                }
            }

            DockerComposeCommands::CargoTest { service, args } => {
                if service.is_none() {
                    ComposeServices::display_available_services();
                    return Ok(());
                }

                let cmd_args = CargoExecService::test(service.clone().unwrap(), args.clone());

                println!("Command execute : {}", cmd_args.join(" "));

                if std::env::var("N7_DRY_RUN").is_ok() {
                    return Ok(());
                }

                let status = Command::new(&cmd_args[0]).args(&cmd_args[1..]).status()?;

                if status.success() {
                    Ok(())
                } else {
                    Err(format!("Command failed with exit code: {:?}", status.code()).into())
                }
            }

            DockerComposeCommands::CargoFmt { service, args } => {
                if service.is_none() {
                    ComposeServices::display_available_services();
                    return Ok(());
                }

                let cmd_args = CargoExecService::fmt(service.clone().unwrap(), args.clone());

                println!("Command execute : {}", cmd_args.join(" "));

                if std::env::var("N7_DRY_RUN").is_ok() {
                    return Ok(());
                }

                let status = Command::new(&cmd_args[0]).args(&cmd_args[1..]).status()?;

                if status.success() {
                    Ok(())
                } else {
                    Err(format!("Command failed with exit code: {:?}", status.code()).into())
                }
            }

            DockerComposeCommands::CargoClippy { service, args } => {
                if service.is_none() {
                    ComposeServices::display_available_services();
                    return Ok(());
                }

                let cmd_args = CargoExecService::clippy(service.clone().unwrap(), args.clone());

                println!("Command execute : {}", cmd_args.join(" "));

                if std::env::var("N7_DRY_RUN").is_ok() {
                    return Ok(());
                }

                let status = Command::new(&cmd_args[0]).args(&cmd_args[1..]).status()?;

                if status.success() {
                    Ok(())
                } else {
                    Err(format!("Command failed with exit code: {:?}", status.code()).into())
                }
            }

            DockerComposeCommands::CargoLlvmCov { service, args } => {
                if service.is_none() {
                    ComposeServices::display_available_services();
                    return Ok(());
                }

                let cmd_args = CargoExecService::llvm_cov(service.clone().unwrap(), args.clone());

                println!("Command execute : {}", cmd_args.join(" "));

                if std::env::var("N7_DRY_RUN").is_ok() {
                    return Ok(());
                }

                let status = Command::new(&cmd_args[0]).args(&cmd_args[1..]).status()?;

                if status.success() {
                    Ok(())
                } else {
                    Err(format!("Command failed with exit code: {:?}", status.code()).into())
                }
            }

            DockerComposeCommands::Rcheck { service } => {
                if service.is_none() {
                    ComposeServices::display_available_services();
                    return Ok(());
                }

                let (fmt_cmd, clippy_cmd, test_cmd) =
                    CargoExecService::rcheck(service.clone().unwrap());

                println!("Running rcheck: fmt -> clippy -> test");

                if std::env::var("N7_DRY_RUN").is_ok() {
                    println!("Command execute : {}", fmt_cmd.join(" "));
                    println!("Command execute : {}", clippy_cmd.join(" "));
                    println!("Command execute : {}", test_cmd.join(" "));
                    println!();
                    println!("{}", "✓ All checks passed successfully!".green().bold());
                    println!("{}", "  → Format (cargo fmt): OK".green());
                    println!("{}", "  → Linter (cargo clippy): OK".green());
                    println!("{}", "  → Tests (cargo test): OK".green());
                    return Ok(());
                }

                println!("Step 1/3: Running cargo fmt...");
                let fmt_status = Command::new(&fmt_cmd[0]).args(&fmt_cmd[1..]).status()?;
                if !fmt_status.success() {
                    return Err(format!(
                        "cargo fmt failed with exit code: {:?}",
                        fmt_status.code()
                    )
                    .into());
                }

                println!("Step 2/3: Running cargo clippy...");
                let clippy_status = Command::new(&clippy_cmd[0])
                    .args(&clippy_cmd[1..])
                    .status()?;
                if !clippy_status.success() {
                    return Err(format!(
                        "cargo clippy failed with exit code: {:?}",
                        clippy_status.code()
                    )
                    .into());
                }

                println!("Step 3/3: Running cargo test...");
                let test_status = Command::new(&test_cmd[0]).args(&test_cmd[1..]).status()?;
                if !test_status.success() {
                    return Err(format!(
                        "cargo test failed with exit code: {:?}",
                        test_status.code()
                    )
                    .into());
                }

                println!();
                println!("{}", "✓ All checks passed successfully!".green().bold());
                println!("{}", "  → Format (cargo fmt): OK".green());
                println!("{}", "  → Linter (cargo clippy): OK".green());
                println!("{}", "  → Tests (cargo test): OK".green());
                Ok(())
            }
        }
    }
}
