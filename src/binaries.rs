use std::env::consts::EXE_EXTENSION;
use std::path::PathBuf;
use std::{env, fs, process};

use anyhow::{Context, Result};

const HASH_LENGTH: usize = 16;

pub struct Binaries {
    root: PathBuf,
    dir: PathBuf,
}

impl Binaries {
    pub fn new() -> Result<Self> {
        let exe = env::current_exe().context("locate current executable")?;
        let bytes = fs::read(&exe)
            .with_context(|| format!(r#"read current executable: "{}""#, exe.display()))?;
        let root = env::temp_dir().join(hash(&bytes));
        let dir = root.join(process::id().to_string());

        fs::create_dir_all(&dir)
            .with_context(|| format!(r#"create binary temp dir: "{}""#, dir.display()))?;

        Ok(Self { root, dir })
    }

    pub fn write(&self, bytes: &[u8]) -> Result<PathBuf> {
        let hash = hash(bytes);
        let path = match EXE_EXTENSION {
            "" => self.dir.join(hash),
            ext => self.dir.join(format!("{hash}.{ext}")),
        };

        fs::write(&path, bytes)
            .with_context(|| format!(r#"write embedded binary: "{}""#, path.display()))?;

        Ok(path)
    }
}

fn hash(bytes: &[u8]) -> String {
    let hash = blake3::hash(bytes).to_hex();

    hash[..HASH_LENGTH].to_owned()
}

impl Drop for Binaries {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
        let _ = fs::remove_dir(&self.root);
    }
}
