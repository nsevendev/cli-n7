use crate::resolvers::tools;
use std::path::PathBuf;

pub struct ComposeFile {}

impl ComposeFile {
    pub fn resolve() -> Option<PathBuf> {
        let candidates = vec![
            "compose.yml",
            "compose.yaml",
            "docker-compose.yml",
            "docker-compose.yaml",
        ];

        tools::search_recursive(".", &candidates, 3)
            .map(|path| path.strip_prefix("./").unwrap_or(&path).to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use crate::resolvers::compose_file::ComposeFile;
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
        fs::create_dir(tmp.path().join("docker")).unwrap();

        // senario 1 => compose.yml
        fs::write(tmp.path().join("compose.yml"), "").unwrap();
        assert_eq!(ComposeFile::resolve(), Some(PathBuf::from("compose.yml")));
        fs::remove_file(tmp.path().join("compose.yml")).unwrap();

        // senario 2 => docker/compose.yml
        fs::write(tmp.path().join("docker/compose.yml"), "").unwrap();
        assert_eq!(
            ComposeFile::resolve(),
            Some(PathBuf::from("docker/compose.yml"))
        );
        fs::remove_file(tmp.path().join("docker/compose.yml")).unwrap();

        // senario 3 => docker/docker-compose.yml
        fs::write(tmp.path().join("docker/docker-compose.yml"), "").unwrap();
        assert_eq!(
            ComposeFile::resolve(),
            Some(PathBuf::from("docker/docker-compose.yml"))
        );
    }
}
