use crate::cargo_cmd::builder::CargoBuilder;

pub struct FmtService {}

impl FmtService {
    pub fn fmt(args: Option<Vec<String>>) -> Vec<String> {
        let mut cmd = CargoBuilder::new().fmt();

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
    fn test_cargo_fmt_without_args() {
        let _lock = n7::test_utils::lock_test();
        let args = FmtService::fmt(None);
        assert_eq!(args, vec!["cargo", "fmt"]);
    }

    #[test]
    fn test_cargo_fmt_with_check() {
        let _lock = n7::test_utils::lock_test();
        let args = FmtService::fmt(Some(vec!["--check".to_string()]));
        assert_eq!(args, vec!["cargo", "fmt", "--check"]);
    }

    #[test]
    fn test_cargo_fmt_with_multiple_args() {
        let _lock = n7::test_utils::lock_test();
        let args = FmtService::fmt(Some(vec!["--check".to_string(), "--verbose".to_string()]));
        assert_eq!(args, vec!["cargo", "fmt", "--check", "--verbose"]);
    }
}
