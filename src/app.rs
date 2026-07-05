use anyhow::Result;
use clap::Parser;

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
        let _ = self.probe.run(&self.args.input)?;

        Ok(())
    }
}
