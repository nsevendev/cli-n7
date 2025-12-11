use std::fs;
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

        Self::search_recursive(".", &candidates, 3)
    }

    fn search_recursive(dir: &str, candidates: &[&str], max_depth: usize) -> Option<PathBuf> {
        if max_depth == 0 {
            return None;
        }

        let ignore_dirs = [".git", "node_modules", "target", ".venv", "vendor", "dist", "build"];

        // Cherche dans le dossier courant
        for candidate in candidates {
            let path = PathBuf::from(dir).join(candidate);
            if path.exists() {
                return Some(path);
            }
        }

        // Cherche dans les sous-dossiers
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let file_name = entry.file_name();

                // Skip les dossiers à ignorer
                if ignore_dirs.iter().any(|&d| file_name == d) {
                    continue;
                }

                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        if let Some(found) = Self::search_recursive(
                            entry.path().to_str()?,
                            candidates,
                            max_depth - 1
                        ) {
                            return Some(found);
                        }
                    }
                }
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
}
