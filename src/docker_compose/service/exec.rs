use crate::docker_compose::builder::DockerComposeBuilder;

pub struct ExecService {}

impl ExecService {
    pub fn exec(service: String, args: Vec<String>) -> Vec<String> {
        let cmd = DockerComposeBuilder::new().exec().add_service(service);

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
        let args = ExecService::exec("app".to_string(), vec!["ls".to_string()]);
        assert_eq!(args, vec!["docker", "compose", "exec", "app", "ls"]);
    }

    #[test]
    fn test_exec_with_command_and_args() {
        let _lock = n7::test_utils::lock_test();
        let args = ExecService::exec("app".to_string(), vec!["ls".to_string(), "-la".to_string()]);
        assert_eq!(args, vec!["docker", "compose", "exec", "app", "ls", "-la"]);
    }

    #[test]
    fn test_exec_with_complex_command() {
        let _lock = n7::test_utils::lock_test();
        let args = ExecService::exec(
            "my_service".to_string(),
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
}
