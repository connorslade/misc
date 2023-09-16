# pdf_splitter

A CLI tool for automatically splitting a PDF into many smaller PDFs by section.
As an example, I split [this PDF](<https://connorcode.com/files/Books/Precalculus%20with%20Limits%20-%20Ron%20Larson%20(2013).pdf>) into

## Usage

```
Usage: pdf_splitter [OPTIONS] <INPUT_FILE> <OUTPUT_DIR> <SHOULD_SPLIT> <RENAME_CAPTURES> <RENAME_FORMAT>

Arguments:
  <INPUT_FILE>       The input PDF file
  <OUTPUT_DIR>       The output directory to write the split PDF files to
  <SHOULD_SPLIT>     Regex to match the section name
  <RENAME_CAPTURES>  Regex to capture the section name
  <RENAME_FORMAT>    Formatter used with `rename_captures` to rename the output files

Options:
  -d, --dry-run                  Dry run, don't save any files
      --allow-unchecked          Don't replace spaces and colons in the section name
  -s, --start-name <START_NAME>  The name of the first section [default: Title]
  -e, --end-name <END_NAME>      The name of the last section [default: End]
  -h, --help                     Print help
  -V, --version                  Print version
```
