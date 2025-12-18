use crate::docker_compose::builder::DockerComposeBuilder;

pub struct ExecService {}

impl ExecService {
    pub fn exec(service: String, detach: bool, args: Vec<String>) -> Vec<String> {
        let mut cmd = DockerComposeBuilder::new().exec();

        if detach {
            cmd = cmd.add_detach();
        }

        cmd = cmd.add_service(service);

        let mut command = DockerComposeBuilder::base_cmd();
        command.extend(cmd.get_args());
        command.extend(args);

        command
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_with_simple_command() {
        let _lock = n7::test_utils::lock_test();
        let args = ExecService::exec("app".to_string(), false, vec!["ls".to_string()]);
        assert_eq!(args, vec!["docker", "compose", "exec", "app", "ls"]);
    }

    #[test]
    fn test_exec_with_command_and_args() {
        let _lock = n7::test_utils::lock_test();
        let args = ExecService::exec(
            "app".to_string(),
            false,
            vec!["ls".to_string(), "-la".to_string()],
        );
        assert_eq!(args, vec!["docker", "compose", "exec", "app", "ls", "-la"]);
    }

    #[test]
    fn test_exec_with_complex_command() {
        let _lock = n7::test_utils::lock_test();
        let args = ExecService::exec(
            "my_service".to_string(),
            false,
            vec![
                "bash".to_string(),
                "-c".to_string(),
                "echo hello".to_string(),
            ],
        );
        assert_eq!(
            args,
            vec![
                "docker",
                "compose",
                "exec",
                "my_service",
                "bash",
                "-c",
                "echo hello"
            ]
        );
    }

    #[test]
    fn test_exec_with_detach_flag() {
        let _lock = n7::test_utils::lock_test();
        let args = ExecService::exec("app".to_string(), true, vec!["ls".to_string()]);
        assert_eq!(args, vec!["docker", "compose", "exec", "-d", "app", "ls"]);
    }

    #[test]
    fn test_exec_with_detach_and_multiple_args() {
        let _lock = n7::test_utils::lock_test();
        let args = ExecService::exec(
            "app".to_string(),
            true,
            vec!["ls".to_string(), "-la".to_string()],
        );
        assert_eq!(
            args,
            vec!["docker", "compose", "exec", "-d", "app", "ls", "-la"]
        );
    }

    #[test]
    fn test_exec_with_detach_and_complex_command() {
        let _lock = n7::test_utils::lock_test();
        let args = ExecService::exec(
            "my_service".to_string(),
            true,
            vec![
                "bash".to_string(),
                "-c".to_string(),
                "echo hello".to_string(),
            ],
        );
        assert_eq!(
            args,
            vec![
                "docker",
                "compose",
                "exec",
                "-d",
                "my_service",
                "bash",
                "-c",
                "echo hello"
            ]
        );
    }
}
