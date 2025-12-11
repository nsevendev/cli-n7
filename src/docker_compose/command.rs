use clap::Subcommand;
use super::service::up::UpService;

#[derive(Subcommand)]
pub enum DockerComposeCommands {
    #[command(name = "u")]
    Up {
        #[arg(short, long)]
        build: bool,

        #[arg(short, long)]
        no_detach: bool,
    }
}

impl DockerComposeCommands {
    pub fn execute(&self) -> Result<(), Box<dyn std::error::Error>> {
        match *self {
            DockerComposeCommands::Up { build, no_detach } => {
                let cmd = UpService::up(build, no_detach);
                println!("{}", cmd.join(" "));
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_up_no_args() {
        let cmd = DockerComposeCommands::Up {
            build: false,
            no_detach: false,
        };

        assert!(cmd.execute().is_ok())
    }
}
