use crate::cargo_cmd::builder::CargoBuilder;

pub struct LlvmCovService {}

impl LlvmCovService {
    pub fn llvm_cov(args: Option<Vec<String>>) -> Vec<String> {
        let mut cmd = CargoBuilder::new().llvm_cov();

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
    fn test_cargo_llvm_cov_without_args() {
        let _lock = n7::test_utils::lock_test();
        let args = LlvmCovService::llvm_cov(None);
        assert_eq!(args, vec!["cargo", "llvm-cov"]);
    }

    #[test]
    fn test_cargo_llvm_cov_with_args() {
        let _lock = n7::test_utils::lock_test();
        let args = LlvmCovService::llvm_cov(Some(vec!["--html".to_string()]));
        assert_eq!(args, vec!["cargo", "llvm-cov", "--html"]);
    }

    #[test]
    fn test_cargo_llvm_cov_with_multiple_args() {
        let _lock = n7::test_utils::lock_test();
        let args = LlvmCovService::llvm_cov(Some(vec!["--html".to_string(), "--open".to_string()]));
        assert_eq!(args, vec!["cargo", "llvm-cov", "--html", "--open"]);
    }
}
