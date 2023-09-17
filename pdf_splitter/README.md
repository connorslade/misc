# pdf_splitter

A CLI tool for automatically splitting a PDF into many smaller PDFs by section.
Useful for when you only need a small segment of a large document at a time and don't want to have to download the full document every time.
As an example, I split the [PDF 1.4 Reference](https://connorcode.com/files/Books/PDF%201.4%20Refrence/pdfreference1.4.pdf) into 19 [separate chapter PDFs](https://connorcode.com/files/Books/PDF%201.4%20Refrence).

> [!WARNING]
> Because I haven't accounted for the entire PDF spec, some PDFs might not work after being split.
> If this happens, you can try converting it to a [PDF/A](https://en.wikipedia.org/wiki/PDF/A) file and trying again.

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

### Example

For the example PDF that I split I used the following command:

`pdf_splitter pdfreference1.4-1.pdf output "(\d|[A-Z]) " "(\d|[A-Z])  (.*)" "$1-$2"`
