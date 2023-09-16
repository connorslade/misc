use std::path::PathBuf;

use clap::Parser;
use regex::Regex;

#[derive(Parser)]
#[clap(
    name = "pdf_splitter",
    version = env!("CARGO_PKG_VERSION"),
    author = "Connor Slade <connor@connorcode.com>",
    about = "Split PDF files by section",    
)]
#[rustfmt::skip]
pub struct Args {
    // == Basic Args ==
    /// The input PDF file.
    pub input_file: PathBuf,
    /// The output directory to write the split PDF files to.
    pub output_dir: PathBuf,
    /// Regex to match the section name.
    pub should_split: Regex,
    /// Regex to capture the section name.
    pub rename_captures: Regex,
    /// Formatter used with `rename_captures` to rename the output files.
    pub rename_format: String,

    // == Flags ==
    /// Dry run, don't save any files.
    #[clap(long, short)]
    pub dry_run: bool,
    
    // == Optional Args ==
    #[clap(long, short, default_value = "Title")]
    pub start_name: String,
    #[clap(long, short, default_value = "End")]
    pub end_name: String,
    /// Doesn't replace spaces and colons in the section name.
    #[clap(long)]
    pub allow_unchecked: bool,
}
