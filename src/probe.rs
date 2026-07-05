use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

use crate::binaries::Binaries;

// TODO?: Blocks other OS builds.
const FFPROBE: &[u8] = include_bytes!("../bin/ffprobe.exe");

pub struct Probe {
    binary: PathBuf,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Output {
    pub streams: Vec<Stream>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Stream {
    pub index: u32,
    pub codec_type: String,

    #[serde(default)]
    pub tags: Tags,
}

#[derive(Default, Deserialize)]
#[allow(dead_code)]
pub struct Tags {
    pub language: Option<String>,
}

impl Probe {
    pub fn new(bin: &Binaries) -> Result<Self> {
        let binary = bin.write(FFPROBE)?;

        Ok(Self { binary })
    }

    pub fn run(&self, input: impl AsRef<Path>) -> Result<Output> {
        let input = input.as_ref();
        let path = input.display();
        let mut command = Command::new(&self.binary);

        command.args([
            "-hide_banner",
            "-v",
            "error",
            "-show_entries",
            "stream=index,codec_type:stream_tags=language",
            "-of",
            "json",
        ]);
        command.arg(input);

        let output = command
            .output()
            .with_context(|| format!(r#"run ffprobe: "{path}""#))?;

        if !output.status.success() {
            bail!(
                "ffprobe exited with error: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }

        serde_json::from_slice(&output.stdout)
            .with_context(|| format!(r#"parse ffprobe output: "{path}""#))
    }
}
