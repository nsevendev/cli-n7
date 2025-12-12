use crate::cargo_cmd::builder::CargoBuilder;

pub struct TestService {}

impl TestService {
    pub fn test(args: Option<Vec<String>>) -> Vec<String> {
        let mut cmd = CargoBuilder::new().test();

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
    fn test_cargo_test_without_args() {
        let _lock = n7::test_utils::lock_test();
        let args = TestService::test(None);
        assert_eq!(args, vec!["cargo", "test"]);
    }

    #[test]
    fn test_cargo_test_with_args() {
        let _lock = n7::test_utils::lock_test();
        let args = TestService::test(Some(vec!["--verbose".to_string()]));
        assert_eq!(args, vec!["cargo", "test", "--verbose"]);
    }

    #[test]
    fn test_cargo_test_with_multiple_args() {
        let _lock = n7::test_utils::lock_test();
        let args = TestService::test(Some(vec![
            "--verbose".to_string(),
            "--nocapture".to_string(),
        ]));
        assert_eq!(args, vec!["cargo", "test", "--verbose", "--nocapture"]);
    }
}
