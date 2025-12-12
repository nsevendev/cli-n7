use super::service::up::UpService;
use crate::docker_compose::service::cargo_exec::CargoExecService;
use crate::docker_compose::service::down::DownService;
use crate::docker_compose::service::logs::LogsService;
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

    #[command(name = "l", about = "\x1b[33mShow logs from services\x1b[0m")]
    Logs {
        #[arg(help = "\x1b[33mService name (optional, default: all services)\x1b[0m")]
        service: Option<String>,

        #[arg(
            short = 'n',
            long = "no-follow",
            help = "\x1b[33mDisable follow mode (default: follow enabled)\x1b[0m"
        )]
        no_follow: bool,
    },

    #[command(name = "c", about = "\x1b[33mExecute custom cargo command\x1b[0m")]
    Cargo {
        #[arg(help = "\x1b[33mDocker service name\x1b[0m")]
        service: String,

        #[arg(last = true, help = "\x1b[33mCargo command and arguments\x1b[0m")]
        args: Vec<String>,
    },

    #[command(name = "ct", about = "\x1b[33mRun cargo test\x1b[0m")]
    CargoTest {
        #[arg(help = "\x1b[33mDocker service name\x1b[0m")]
        service: String,

        #[arg(
            last = true,
            help = "\x1b[33mAdditional arguments for cargo test\x1b[0m"
        )]
        args: Option<Vec<String>>,
    },

    #[command(name = "cf", about = "\x1b[33mRun cargo fmt\x1b[0m")]
    CargoFmt {
        #[arg(help = "\x1b[33mDocker service name\x1b[0m")]
        service: String,

        #[arg(
            last = true,
            help = "\x1b[33mAdditional arguments for cargo fmt\x1b[0m"
        )]
        args: Option<Vec<String>>,
    },

    #[command(name = "cc", about = "\x1b[33mRun cargo clippy\x1b[0m")]
    CargoClippy {
        #[arg(help = "\x1b[33mDocker service name\x1b[0m")]
        service: String,

        #[arg(
            last = true,
            help = "\x1b[33mAdditional arguments for cargo clippy\x1b[0m"
        )]
        args: Option<Vec<String>>,
    },

    #[command(
        name = "rcheck",
        about = "\x1b[33mRun fmt, clippy, and test in sequence\x1b[0m"
    )]
    Rcheck {
        #[arg(help = "\x1b[33mDocker service name\x1b[0m")]
        service: String,
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

            DockerComposeCommands::Cargo { service, args } => {
                let cmd_args = CargoExecService::cargo(service.clone(), args.clone());

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
                let cmd_args = CargoExecService::test(service.clone(), args.clone());

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
                let cmd_args = CargoExecService::fmt(service.clone(), args.clone());

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
                let cmd_args = CargoExecService::clippy(service.clone(), args.clone());

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
                let (fmt_cmd, clippy_cmd, test_cmd) = CargoExecService::rcheck(service.clone());

                println!("Running rcheck: fmt -> clippy -> test");

                if std::env::var("N7_DRY_RUN").is_ok() {
                    println!("Command execute : {}", fmt_cmd.join(" "));
                    println!("Command execute : {}", clippy_cmd.join(" "));
                    println!("Command execute : {}", test_cmd.join(" "));
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

                println!("All checks passed!");
                Ok(())
            }
        }
    }
}
