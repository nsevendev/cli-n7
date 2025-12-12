use crate::cargo_cmd::{ClippyService, FmtService, TestService};
use crate::docker_compose::builder::DockerComposeBuilder;

pub struct CargoExecService {}

impl CargoExecService {
    pub fn test(service: String, args: Option<Vec<String>>) -> Vec<String> {
        let cargo_args = TestService::test(args);
        Self::build_exec_command(service, cargo_args)
    }

    pub fn fmt(service: String, args: Option<Vec<String>>) -> Vec<String> {
        let cargo_args = FmtService::fmt(args);
        Self::build_exec_command(service, cargo_args)
    }

    pub fn clippy(service: String, args: Option<Vec<String>>) -> Vec<String> {
        let cargo_args = ClippyService::clippy(args);
        Self::build_exec_command(service, cargo_args)
    }

    pub fn cargo(service: String, args: Vec<String>) -> Vec<String> {
        let mut cargo_cmd = vec!["cargo".to_string()];
        cargo_cmd.extend(args);
        Self::build_exec_command(service, cargo_cmd)
    }

    pub fn rcheck(service: String) -> (Vec<String>, Vec<String>, Vec<String>) {
        let fmt_cmd = Self::fmt(service.clone(), None);
        let clippy_cmd = Self::clippy(service.clone(), None);
        let test_cmd = Self::test(service, None);

        (fmt_cmd, clippy_cmd, test_cmd)
    }

    fn build_exec_command(service: String, cargo_args: Vec<String>) -> Vec<String> {
        let cmd = DockerComposeBuilder::new().exec().add_service(service);

        let mut args = DockerComposeBuilder::base_cmd();
        args.extend(cmd.get_args());
        args.extend(cargo_args);

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_exec_test_without_args() {
        let _lock = n7::test_utils::lock_test();
        let args = CargoExecService::test("my_service".to_string(), None);
        assert_eq!(
            args,
            vec!["docker", "compose", "exec", "my_service", "cargo", "test"]
        );
    }

    #[test]
    fn test_cargo_exec_test_with_args() {
        let _lock = n7::test_utils::lock_test();
        let args = CargoExecService::test(
            "my_service".to_string(),
            Some(vec!["--verbose".to_string()]),
        );
        assert_eq!(
            args,
            vec![
                "docker",
                "compose",
                "exec",
                "my_service",
                "cargo",
                "test",
                "--verbose"
            ]
        );
    }

    #[test]
    fn test_cargo_exec_fmt() {
        let _lock = n7::test_utils::lock_test();
        let args = CargoExecService::fmt("app".to_string(), None);
        assert_eq!(
            args,
            vec!["docker", "compose", "exec", "app", "cargo", "fmt"]
        );
    }

    #[test]
    fn test_cargo_exec_clippy() {
        let _lock = n7::test_utils::lock_test();
        let args = CargoExecService::clippy("app".to_string(), None);
        assert_eq!(
            args,
            vec!["docker", "compose", "exec", "app", "cargo", "clippy"]
        );
    }

    #[test]
    fn test_cargo_exec_cargo_with_custom_command() {
        let _lock = n7::test_utils::lock_test();
        let args = CargoExecService::cargo(
            "app".to_string(),
            vec!["build".to_string(), "--release".to_string()],
        );
        assert_eq!(
            args,
            vec![
                "docker",
                "compose",
                "exec",
                "app",
                "cargo",
                "build",
                "--release"
            ]
        );
    }

    #[test]
    fn test_cargo_exec_rcheck() {
        let _lock = n7::test_utils::lock_test();
        let (fmt_cmd, clippy_cmd, test_cmd) = CargoExecService::rcheck("app".to_string());

        assert_eq!(
            fmt_cmd,
            vec!["docker", "compose", "exec", "app", "cargo", "fmt"]
        );
        assert_eq!(
            clippy_cmd,
            vec!["docker", "compose", "exec", "app", "cargo", "clippy"]
        );
        assert_eq!(
            test_cmd,
            vec!["docker", "compose", "exec", "app", "cargo", "test"]
        );
    }
}
