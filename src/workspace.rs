use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Workspace {
    pub root: PathBuf,
}

impl Workspace {
    pub fn discover(start: &Path) -> Self {
        let mut current = if start.is_dir() {
            start.to_path_buf()
        } else {
            start.parent().unwrap_or(start).to_path_buf()
        };
        loop {
            if current.join("nox.build").is_file() || current.join("noxfile").is_file() {
                return Self { root: current };
            }
            if !current.pop() {
                break;
            }
        }
        Self {
            root: start.to_path_buf(),
        }
    }
}
