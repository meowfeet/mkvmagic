use std::path::Path;

use anyhow::Result;
use clap::Parser;
use walkdir::WalkDir;

use crate::binaries::Binaries;
use crate::cli::Args;
use crate::probe::Probe;

pub struct App {
    args: Args,
    _binaries: Binaries,
    probe: Probe,
}

impl App {
    pub fn new() -> Result<Self> {
        let args = Args::parse();
        let binaries = Binaries::new()?;
        let probe = Probe::new(&binaries)?;

        Ok(Self {
            args,
            _binaries: binaries,
            probe,
        })
    }

    pub fn run(&self) -> Result<()> {
        for entry in WalkDir::new(&self.args.input) {
            let entry = entry?;
            if entry.file_type().is_file() && is_mkv(entry.path()) {
                self.probe.run(entry.path())?;
            }
        }

        Ok(())
    }
}

fn is_mkv(path: impl AsRef<Path>) -> bool {
    path.as_ref().extension().is_some_and(|ext| ext.eq_ignore_ascii_case("mkv"))
}