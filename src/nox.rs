use std::{path::Path, process::Command};

pub struct NoxOutput {
    pub status: Option<i32>,
    pub text: String,
}

pub fn run(workspace: &Path, command: &str) -> std::io::Result<NoxOutput> {
    let output = Command::new("nox")
        .arg(command)
        .current_dir(workspace)
        .output()?;
    Ok(NoxOutput {
        status: output.status.code(),
        text: format!(
            "{}{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ),
    })
}
