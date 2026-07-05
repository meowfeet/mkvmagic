use std::path::PathBuf;

use clap::Parser;

/// Rename and clean up MKV files
#[derive(Parser)]
#[command(version)]
pub struct Args {
    /// File or directory to process
    pub input: PathBuf,

    /// Destination file or directory
    pub output: PathBuf,

    /// TMDB movie ID for renaming
    #[arg(short = 'm', long, value_name = "ID", conflicts_with = "tvdb")]
    pub tmdb: Option<String>,

    /// TVDB show ID for renaming
    #[arg(short = 's', long, value_name = "ID", conflicts_with = "tmdb")]
    pub tvdb: Option<String>,

    /// Replace existing destination
    #[arg(short = 'y', long)]
    pub overwrite: bool,
}
