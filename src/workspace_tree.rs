use std::{fs, path::PathBuf};

#[derive(Debug, Clone)]
pub struct TreeNode {
    pub path: PathBuf,
    pub name: String,
    pub directory: bool,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn workspace(root: PathBuf) -> Self {
        Self {
            name: root.display().to_string(),
            path: root.clone(),
            directory: true,
            children: children(&root),
        }
    }
}

fn children(path: &PathBuf) -> Vec<TreeNode> {
    let mut nodes: Vec<_> = fs::read_dir(path)
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .filter_map(|entry| {
            let child = entry.path();
            let name = child.file_name()?.to_string_lossy().into_owned();
            if matches!(name.as_str(), ".git" | "build" | "target") {
                return None;
            }
            let directory = child.is_dir();
            Some(TreeNode {
                name,
                path: child.clone(),
                directory,
                children: if directory {
                    children(&child)
                } else {
                    Vec::new()
                },
            })
        })
        .collect();
    nodes.sort_by_key(|node| (!node.directory, node.name.to_lowercase()));
    nodes
}
