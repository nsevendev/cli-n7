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
}
