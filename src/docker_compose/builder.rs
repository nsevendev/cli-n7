pub(super) struct DockerComposeBuilder {
    args: Vec<String>,
}

#[allow(dead_code)] // pour ne pas affiche dans la console les fonctions non utilisées
impl DockerComposeBuilder {
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }

    pub fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }

    pub fn base_cmd() -> Vec<String> {
        vec!["docker".to_string(), "compose".to_string()]
    }

    pub fn up(mut self) -> Self {
        self.args.extend(vec!["up".to_string(), "-d".to_string()]);
        self
    }

    pub fn down(mut self) -> Self {
        self.args.extend(vec!["down".to_string()]);
        self
    }

    pub fn exec(mut self) -> Self {
        self.args.extend(vec!["exec".to_string()]);
        self
    }

    pub fn logs(mut self) -> Self {
        self.args.extend(vec!["logs".to_string()]);
        self
    }

    pub fn volumes(mut self) -> Self {
        self.args.push("-v".to_string());
        self
    }

    pub fn remove_orphan(mut self) -> Self {
        self.args.push("--remove-orphans".to_string());
        self
    }

    pub fn no_detach(mut self) -> Self {
        self.args.retain(|arg| arg != "-d");
        self
    }

    pub fn add_build(mut self) -> Self {
        if let Some(pos) = self.args.iter().position(|arg| arg == "-d") {
            self.args.insert(pos, "--build".to_string());
        } else {
            self.args.push("--build".to_string());
        }
        self
    }

    pub fn add_path_file_env(mut self, envname: String) -> Self {
        self.args
            .extend(vec!["--env-file".to_string(), envname.to_string()]);
        self
    }

    pub fn add_path_file_compose(mut self, path: String) -> Self {
        self.args.extend(vec!["-f".to_string(), path]);
        self
    }

    pub fn add_service(mut self, service: String) -> Self {
        self.args.push(service);
        self
    }

    pub fn add_shell(mut self, shell: String) -> Self {
        self.args.push(shell);
        self
    }

    pub fn add_follow(mut self) -> Self {
        self.args.push("-f".to_string());
        self
    }

    pub fn add_detach(mut self) -> Self {
        self.args.push("-d".to_string());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_init_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new();
        assert_eq!(dcb.args, Vec::<String>::new());
    }

    #[test]
    fn create_up_and_get_args() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new().up();
        assert_eq!(dcb.args, dcb.get_args());
    }

    #[test]
    pub fn test_base_cmd() {
        let _lock = n7::test_utils::lock_test();
        let cmd = vec!["docker".to_string(), "compose".to_string()];
        let res = DockerComposeBuilder::base_cmd();

        assert_eq!(res, cmd, "Is equal to docker compose command?");
    }

    #[test]
    fn create_up_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new().up();
        assert_eq!(dcb.args, vec!["up", "-d"]);
    }

    #[test]
    fn create_down_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new().down();
        assert_eq!(dcb.args, vec!["down"]);
    }

    #[test]
    fn create_down_with_volume_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new().down().volumes();
        assert_eq!(dcb.args, vec!["down", "-v"]);
    }

    #[test]
    fn create_down_remove_orphan_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new().down().remove_orphan();
        assert_eq!(dcb.args, vec!["down", "--remove-orphans"]);
    }

    #[test]
    fn create_down_with_volume_remove_orphan_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new().down().volumes().remove_orphan();
        assert_eq!(dcb.args, vec!["down", "-v", "--remove-orphans"]);
    }

    #[test]
    fn create_up_with_build_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new().up().add_build();
        assert_eq!(dcb.args, vec!["up", "--build", "-d"]);
    }

    #[test]
    fn create_up_no_detach_with_build_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new().up().no_detach().add_build();
        assert_eq!(dcb.args, vec!["up", "--build"]);
    }

    #[test]
    fn create_up_no_detached_with_build_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new().up().add_build().no_detach();
        assert_eq!(dcb.args, vec!["up", "--build"]);
    }

    #[test]
    fn create_up_no_detached_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new().up().no_detach();
        assert_eq!(dcb.args, vec!["up"]);
    }

    #[test]
    fn create_up_with_env_file_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new()
            .add_path_file_env(".env".to_string())
            .up();
        assert_eq!(dcb.args, vec!["--env-file", ".env", "up", "-d"]);
    }

    #[test]
    fn create_up_with_env_file_and_with_path_compose_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new()
            .add_path_file_env(".env".to_string())
            .add_path_file_compose("docker/compose.yml".to_string())
            .up();
        assert_eq!(
            dcb.args,
            vec!["--env-file", ".env", "-f", "docker/compose.yml", "up", "-d"]
        );
    }

    #[test]
    fn create_exec_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new().exec();
        assert_eq!(dcb.args, vec!["exec"]);
    }

    #[test]
    fn create_exec_with_service_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new()
            .exec()
            .add_service("my_service".to_string());
        assert_eq!(dcb.args, vec!["exec", "my_service"]);
    }

    #[test]
    fn create_exec_with_service_and_shell_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new()
            .exec()
            .add_service("my_service".to_string())
            .add_shell("bash".to_string());
        assert_eq!(dcb.args, vec!["exec", "my_service", "bash"]);
    }

    #[test]
    fn create_exec_with_service_and_custom_shell_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new()
            .exec()
            .add_service("app".to_string())
            .add_shell("sh".to_string());
        assert_eq!(dcb.args, vec!["exec", "app", "sh"]);
    }

    #[test]
    fn create_logs_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new().logs();
        assert_eq!(dcb.args, vec!["logs"]);
    }

    #[test]
    fn create_logs_with_follow_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new().logs().add_follow();
        assert_eq!(dcb.args, vec!["logs", "-f"]);
    }

    #[test]
    fn create_logs_with_service_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new()
            .logs()
            .add_service("my_service".to_string());
        assert_eq!(dcb.args, vec!["logs", "my_service"]);
    }

    #[test]
    fn create_logs_with_follow_and_service_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new()
            .logs()
            .add_follow()
            .add_service("app".to_string());
        assert_eq!(dcb.args, vec!["logs", "-f", "app"]);
    }

    #[test]
    fn create_exec_with_detach_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new().exec().add_detach();
        assert_eq!(dcb.args, vec!["exec", "-d"]);
    }

    #[test]
    fn create_exec_with_detach_and_service_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new()
            .exec()
            .add_detach()
            .add_service("app".to_string());
        assert_eq!(dcb.args, vec!["exec", "-d", "app"]);
    }

    #[test]
    fn create_exec_with_detach_service_and_shell_docker_compose_builder() {
        let _lock = n7::test_utils::lock_test();
        let dcb = DockerComposeBuilder::new()
            .exec()
            .add_detach()
            .add_service("my_service".to_string())
            .add_shell("bash".to_string());
        assert_eq!(dcb.args, vec!["exec", "-d", "my_service", "bash"]);
    }
}
