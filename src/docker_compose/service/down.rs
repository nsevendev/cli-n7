use crate::docker_compose::builder::DockerComposeBuilder;

pub struct DownService {}

impl DownService {
    pub fn down(rmvolumes: bool, rmorphans: bool) -> Vec<String> {
        let mut cmd = DockerComposeBuilder::new().down();

        if rmvolumes {
            cmd = cmd.volumes()
        }

        if rmorphans {
            cmd = cmd.remove_orphan()
        }

        let mut args = DockerComposeBuilder::base_cmd();
        args.extend(cmd.get_args());

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_down() {
        let args = DownService::down(false, false);
        assert_eq!(args, vec!["docker", "compose", "down"]);
    }

    #[test]
    fn test_down_with_rmvolume() {
        let args = DownService::down(true, false);
        assert_eq!(args, vec!["docker", "compose", "down", "-v"]);
    }

    #[test]
    fn test_down_with_rmorphans() {
        let args = DownService::down(false, true);
        assert_eq!(args, vec!["docker", "compose", "down", "--remove-orphans"]);
    }

    #[test]
    fn test_down_with_all_args() {
        let args = DownService::down(true, true);
        assert_eq!(
            args,
            vec!["docker", "compose", "down", "-v", "--remove-orphans"]
        );
    }
}
