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
    /// Don't replace spaces and colons in the section name.
    #[clap(long)]
    pub allow_unchecked: bool,

    // == Optional Args ==
    /// The name of the first section.
    #[clap(long, short, default_value = "Title")]
    pub start_name: String,
    /// The name of the last section.
    #[clap(long, short, default_value = "End")]
    pub end_name: String,
    /// The depth of heading to search through.
    /// Setting to -1 will include all headings.
    #[clap(long, default_value_t = {0})]
    pub depth: usize
}
