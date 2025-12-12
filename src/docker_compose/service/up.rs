use crate::docker_compose::builder::DockerComposeBuilder;
use crate::resolvers::{ComposeFile, EnvFile};

pub struct UpService {}

impl UpService {
    pub fn up(
        build: bool,
        no_detach: bool,
        env_file: Option<String>,
        compose_file: Option<String>,
    ) -> Vec<String> {
        let mut cmd = DockerComposeBuilder::new();

        // env file
        let env_path =
            env_file.or_else(|| EnvFile::resolve().map(|p| p.to_string_lossy().to_string()));
        if let Some(path) = env_path {
            cmd = cmd.add_path_file_env(path);
        }

        // compose file
        let compose_path = compose_file
            .or_else(|| ComposeFile::resolve().map(|p| p.to_string_lossy().to_string()));
        if let Some(path) = compose_path {
            cmd = cmd.add_path_file_compose(path);
        }

        cmd = cmd.up();

        if build {
            cmd = cmd.add_build();
        }

        if no_detach {
            cmd = cmd.no_detach();
        }

        let mut args = DockerComposeBuilder::base_cmd();
        args.extend(cmd.get_args());

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use tempfile::TempDir;

    fn setup() -> TempDir {
        let tmp = TempDir::new().unwrap();
        env::set_current_dir(tmp.path()).unwrap();
        tmp
    }

    #[test]
    pub fn test_up_with_args_false() {
        let _lock = n7::test_utils::lock_test();
        let _tmp = setup();

        let cmd = vec![
            "docker".to_string(),
            "compose".to_string(),
            "up".to_string(),
            "-d".to_string(),
        ];

        let res = UpService::up(false, false, None, None);

        assert!(res.len() > 0);
        assert_eq!(res, cmd)
    }

    #[test]
    pub fn test_up_with_args_build_true() {
        let _lock = n7::test_utils::lock_test();
        let _tmp = setup();

        let cmd = vec![
            "docker".to_string(),
            "compose".to_string(),
            "up".to_string(),
            "--build".to_string(),
            "-d".to_string(),
        ];

        let res = UpService::up(true, false, None, None);

        assert!(res.len() > 0);
        assert_eq!(res, cmd)
    }

    #[test]
    pub fn test_up_with_args_no_detach_true() {
        let _lock = n7::test_utils::lock_test();
        let _tmp = setup();

        let cmd = vec![
            "docker".to_string(),
            "compose".to_string(),
            "up".to_string(),
        ];

        let res = UpService::up(false, true, None, None);

        assert!(res.len() > 0);
        assert_eq!(res, cmd)
    }

    #[test]
    pub fn test_up_with_args_build_and_no_detach_true() {
        let _lock = n7::test_utils::lock_test();
        let _tmp = setup();

        let cmd = vec![
            "docker".to_string(),
            "compose".to_string(),
            "up".to_string(),
            "--build".to_string(),
        ];

        let res = UpService::up(true, true, None, None);

        assert!(res.len() > 0);
        assert_eq!(res, cmd)
    }
}
