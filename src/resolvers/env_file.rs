use crate::resolvers::tools;
use std::path::PathBuf;

pub struct EnvFile {}

impl EnvFile {
    pub fn resolve() -> Option<PathBuf> {
        let candidates = vec![
            ".env",
            ".env.dev",
            ".env.prod",
            ".env.preprod",
            ".env.staging",
            ".env.api",
            ".env.front",
            ".env.back",
        ];

        tools::search_recursive(".", &candidates, 3)
            .map(|path| path.strip_prefix("./").unwrap_or(&path).to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use crate::resolvers::env_file::EnvFile;
    use std::path::PathBuf;
    use std::{env, fs};
    use tempfile::TempDir;

    fn setup() -> TempDir {
        let tmp = TempDir::new().unwrap();
        env::set_current_dir(tmp.path()).unwrap();
        tmp
    }

    #[test]
    fn test_resolve() {
        let _lock = n7::test_utils::lock_test();

        let tmp = setup();
        fs::create_dir(tmp.path().join("api")).unwrap();

        // senario 1 => compose.yml
        fs::write(tmp.path().join(".env"), "").unwrap();
        assert_eq!(EnvFile::resolve(), Some(PathBuf::from(".env")));
        fs::remove_file(tmp.path().join(".env")).unwrap();

        // senario 2 => docker/compose.yml
        fs::write(tmp.path().join(".env.api"), "").unwrap();
        assert_eq!(EnvFile::resolve(), Some(PathBuf::from(".env.api")));
        fs::remove_file(tmp.path().join(".env.api")).unwrap();

        // senario 3 => docker/docker-compose.yml
        fs::write(tmp.path().join("api/.env"), "").unwrap();
        assert_eq!(EnvFile::resolve(), Some(PathBuf::from("api/.env")));
    }
}
