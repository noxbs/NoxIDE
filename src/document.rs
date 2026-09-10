use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct Document {
    pub path: PathBuf,
    pub text: String,
    pub dirty: bool,
}

impl Document {
    pub fn open(path: PathBuf) -> std::io::Result<Self> {
        Ok(Self {
            text: fs::read_to_string(&path)?,
            path,
            dirty: false,
        })
    }

    pub fn save(&mut self) -> std::io::Result<()> {
        fs::write(&self.path, &self.text)?;
        self.dirty = false;
        Ok(())
    }
}
