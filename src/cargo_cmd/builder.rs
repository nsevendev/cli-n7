pub struct CargoBuilder {
    args: Vec<String>,
}

#[allow(dead_code)]
impl CargoBuilder {
    pub fn new() -> Self {
        Self { args: Vec::new() }
    }

    pub fn get_args(&self) -> Vec<String> {
        self.args.clone()
    }

    pub fn base_cmd() -> Vec<String> {
        vec!["cargo".to_string()]
    }

    pub fn test(mut self) -> Self {
        self.args.push("test".to_string());
        self
    }

    pub fn fmt(mut self) -> Self {
        self.args.push("fmt".to_string());
        self
    }

    pub fn clippy(mut self) -> Self {
        self.args.push("clippy".to_string());
        self
    }

    pub fn add_args(mut self, args: Vec<String>) -> Self {
        self.args.extend(args);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_init_cargo_builder() {
        let _lock = n7::test_utils::lock_test();
        let cb = CargoBuilder::new();
        assert_eq!(cb.args, Vec::<String>::new());
    }

    #[test]
    fn test_base_cmd() {
        let _lock = n7::test_utils::lock_test();
        let cmd = vec!["cargo".to_string()];
        let res = CargoBuilder::base_cmd();
        assert_eq!(res, cmd);
    }

    #[test]
    fn create_test_cargo_builder() {
        let _lock = n7::test_utils::lock_test();
        let cb = CargoBuilder::new().test();
        assert_eq!(cb.args, vec!["test"]);
    }

    #[test]
    fn create_fmt_cargo_builder() {
        let _lock = n7::test_utils::lock_test();
        let cb = CargoBuilder::new().fmt();
        assert_eq!(cb.args, vec!["fmt"]);
    }

    #[test]
    fn create_clippy_cargo_builder() {
        let _lock = n7::test_utils::lock_test();
        let cb = CargoBuilder::new().clippy();
        assert_eq!(cb.args, vec!["clippy"]);
    }

    #[test]
    fn create_test_with_args_cargo_builder() {
        let _lock = n7::test_utils::lock_test();
        let cb = CargoBuilder::new()
            .test()
            .add_args(vec!["--verbose".to_string()]);
        assert_eq!(cb.args, vec!["test", "--verbose"]);
    }

    #[test]
    fn create_fmt_with_args_cargo_builder() {
        let _lock = n7::test_utils::lock_test();
        let cb = CargoBuilder::new()
            .fmt()
            .add_args(vec!["--check".to_string()]);
        assert_eq!(cb.args, vec!["fmt", "--check"]);
    }

    #[test]
    fn create_clippy_with_args_cargo_builder() {
        let _lock = n7::test_utils::lock_test();
        let cb = CargoBuilder::new()
            .clippy()
            .add_args(vec!["--fix".to_string()]);
        assert_eq!(cb.args, vec!["clippy", "--fix"]);
    }
}
