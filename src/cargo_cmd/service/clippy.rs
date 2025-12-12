use crate::cargo_cmd::builder::CargoBuilder;

pub struct ClippyService {}

impl ClippyService {
    pub fn clippy(args: Option<Vec<String>>) -> Vec<String> {
        let mut cmd = CargoBuilder::new().clippy();

        if let Some(extra_args) = args {
            cmd = cmd.add_args(extra_args);
        }

        let mut command = CargoBuilder::base_cmd();
        command.extend(cmd.get_args());

        command
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cargo_clippy_without_args() {
        let _lock = n7::test_utils::lock_test();
        let args = ClippyService::clippy(None);
        assert_eq!(args, vec!["cargo", "clippy"]);
    }

    #[test]
    fn test_cargo_clippy_with_fix() {
        let _lock = n7::test_utils::lock_test();
        let args = ClippyService::clippy(Some(vec!["--fix".to_string()]));
        assert_eq!(args, vec!["cargo", "clippy", "--fix"]);
    }

    #[test]
    fn test_cargo_clippy_with_multiple_args() {
        let _lock = n7::test_utils::lock_test();
        let args = ClippyService::clippy(Some(vec![
            "--".to_string(),
            "-D".to_string(),
            "warnings".to_string(),
        ]));
        assert_eq!(args, vec!["cargo", "clippy", "--", "-D", "warnings"]);
    }
}
