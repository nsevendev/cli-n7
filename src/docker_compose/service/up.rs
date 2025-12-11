use crate::docker_compose::builder::DockerComposeBuilder;

pub struct UpService {}

impl UpService {
    pub fn up(build: bool, no_detach: bool) -> Vec<String> {
        let mut cmd = DockerComposeBuilder::new().up();

        if build {
            cmd = cmd.add_build();
        }

        if no_detach {
            cmd = cmd.no_detach()
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
    pub fn test_up_with_args_false() {
        let cmd = vec![
            "docker".to_string(),
            "compose".to_string(),
            "up".to_string(),
            "-d".to_string()
        ];

        let res = UpService::up(false, false);

        assert!(res.len() > 0);
        assert_eq!(res, cmd)
    }

    #[test]
    pub fn test_up_with_args_build_true() {
        let cmd = vec![
            "docker".to_string(),
            "compose".to_string(),
            "up".to_string(),
            "--build".to_string(),
            "-d".to_string()
        ];

        let res = UpService::up(true, false);

        assert!(res.len() > 0);
        assert_eq!(res, cmd)
    }

    #[test]
    pub fn test_up_with_args_no_detach_true() {
        let cmd = vec![
            "docker".to_string(),
            "compose".to_string(),
            "up".to_string(),
        ];

        let res = UpService::up(false, true);

        assert!(res.len() > 0);
        assert_eq!(res, cmd)
    }

    #[test]
    pub fn test_up_with_args_build_and_no_detach_true() {
        let cmd = vec![
            "docker".to_string(),
            "compose".to_string(),
            "up".to_string(),
            "--build".to_string(),
        ];

        let res = UpService::up(true, true);

        assert!(res.len() > 0);
        assert_eq!(res, cmd)
    }
}
