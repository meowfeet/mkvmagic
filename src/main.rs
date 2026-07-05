mod app;
mod binaries;
mod cli;
mod probe;

use anyhow::Result;
use app::App;

fn main() -> Result<()> {
    App::new()?.run()
}
