use crate::docker_compose::builder::DockerComposeBuilder;

pub struct LogsService {}

impl LogsService {
    pub fn logs(service: Option<String>, follow: bool) -> Vec<String> {
        let mut cmd = DockerComposeBuilder::new().logs();

        if follow {
            cmd = cmd.add_follow();
        }

        if let Some(svc) = service {
            cmd = cmd.add_service(svc);
        }

        let mut args = DockerComposeBuilder::base_cmd();
        args.extend(cmd.get_args());

        args
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logs_all_services_with_follow() {
        let _lock = n7::test_utils::lock_test();
        let args = LogsService::logs(None, true);
        assert_eq!(args, vec!["docker", "compose", "logs", "-f"]);
    }

    #[test]
    fn test_logs_all_services_without_follow() {
        let _lock = n7::test_utils::lock_test();
        let args = LogsService::logs(None, false);
        assert_eq!(args, vec!["docker", "compose", "logs"]);
    }

    #[test]
    fn test_logs_specific_service_with_follow() {
        let _lock = n7::test_utils::lock_test();
        let args = LogsService::logs(Some("my_service".to_string()), true);
        assert_eq!(args, vec!["docker", "compose", "logs", "-f", "my_service"]);
    }

    #[test]
    fn test_logs_specific_service_without_follow() {
        let _lock = n7::test_utils::lock_test();
        let args = LogsService::logs(Some("my_service".to_string()), false);
        assert_eq!(args, vec!["docker", "compose", "logs", "my_service"]);
    }
}
