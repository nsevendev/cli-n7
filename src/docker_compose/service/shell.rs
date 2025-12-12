use crate::docker_compose::builder::DockerComposeBuilder;

pub struct ShellService {}

impl ShellService {
    pub fn shell(service: String, shell: Option<String>) -> Vec<String> {
        let shell_cmd = shell.unwrap_or_else(|| "bash".to_string());

        let cmd = DockerComposeBuilder::new()
            .exec()
            .add_service(service)
            .add_shell(shell_cmd);

        let mut args = DockerComposeBuilder::base_cmd();
        args.extend(cmd.get_args());

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_with_service_name_default_bash() {
        let _lock = n7::test_utils::lock_test();
        let args = ShellService::shell("my_service".to_string(), None);
        assert_eq!(
            args,
            vec!["docker", "compose", "exec", "my_service", "bash"]
        );
    }

    #[test]
    fn test_shell_with_service_name_and_custom_shell() {
        let _lock = n7::test_utils::lock_test();
        let args = ShellService::shell("my_service".to_string(), Some("sh".to_string()));
        assert_eq!(args, vec!["docker", "compose", "exec", "my_service", "sh"]);
    }
}
