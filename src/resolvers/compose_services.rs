use super::compose_file::ComposeFile;
use colored::Colorize;
use serde_yaml::Value;
use std::fs;

pub struct ComposeServices;

impl ComposeServices {
    pub fn list() -> Result<Vec<String>, String> {
        let compose_path =
            ComposeFile::resolve().ok_or("No docker-compose file found in current directory")?;

        let content = fs::read_to_string(&compose_path)
            .map_err(|e| format!("Failed to read compose file: {}", e))?;

        let yaml: Value = serde_yaml::from_str(&content)
            .map_err(|e| format!("Failed to parse compose file: {}", e))?;

        let services = yaml
            .get("services")
            .and_then(|s| s.as_mapping())
            .ok_or("No 'services' section found in compose file")?;

        let service_names: Vec<String> = services
            .keys()
            .filter_map(|k| k.as_str().map(|s| s.to_string()))
            .collect();

        if service_names.is_empty() {
            return Err("No services found in compose file".to_string());
        }

        Ok(service_names)
    }

    pub fn display_available_services() {
        match Self::list() {
            Ok(services) => {
                println!(
                    "\n{} {}",
                    "📦".blue(),
                    "Available services in docker-compose file:".cyan().bold()
                );
                println!("{}", "────────────────────────────────────────────".blue());
                for service in services {
                    println!("  {} {}", "•".green(), service.yellow());
                }
                println!("{}", "────────────────────────────────────────────".blue());
                println!(
                    "\n{} {}\n",
                    "💡".yellow(),
                    "Usage: n7 dc <command> <service_name>".white().dimmed()
                );
            }
            Err(e) => {
                eprintln!("{}", format!("Error: {}", e).red());
                std::process::exit(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_compose_file(dir: &TempDir, content: &str) {
        let compose_path = dir.path().join("docker-compose.yml");
        let mut file = File::create(compose_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    #[test]
    fn test_list_services_success() {
        let _lock = n7::test_utils::lock_test();
        let tmp = TempDir::new().unwrap();
        env::set_current_dir(tmp.path()).unwrap();

        let compose_content = r#"
version: '3'
services:
  web:
    image: nginx
  db:
    image: postgres
  cache:
    image: redis
"#;
        create_test_compose_file(&tmp, compose_content);

        let services = ComposeServices::list().unwrap();
        assert_eq!(services.len(), 3);
        assert!(services.contains(&"web".to_string()));
        assert!(services.contains(&"db".to_string()));
        assert!(services.contains(&"cache".to_string()));
    }

    #[test]
    fn test_list_services_no_file() {
        let _lock = n7::test_utils::lock_test();
        let tmp = TempDir::new().unwrap();
        env::set_current_dir(tmp.path()).unwrap();

        let result = ComposeServices::list();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No docker-compose file found"));
    }

    #[test]
    fn test_list_services_empty() {
        let _lock = n7::test_utils::lock_test();
        let tmp = TempDir::new().unwrap();
        env::set_current_dir(tmp.path()).unwrap();

        let compose_content = r#"
version: '3'
services: {}
"#;
        create_test_compose_file(&tmp, compose_content);

        let result = ComposeServices::list();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No services found"));
    }
}
