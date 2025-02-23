# Code Snip

`code-snip` is a Rust binary that simplifies the process of generating code snippets. It allows you to quickly create and save code snippets by specifying the input code and the desired output file name.

## Installation

Install `code-snip` via Cargo:

```sh
cargo install code-snip
```

## Usage

```sh
code-snip --input "fn main() {
    println!(\"Hello world\");
}" --output snippet.png
```

![Example output image](https://github.com/max-taylor/code-snip/blob/main/src/assets/example.png)

## Arguments

- `input`: The code you want to create a snippet for.
- `output`: The filename for the generated snippet.

## Environment Variables

You can configure the export path via the `SYNTAX_EXPORT_PATH` environment variable. If set, all generated snippets will be saved in this directory:

```sh
export SYNTAX_EXPORT_PATH="/path/to/snippets"
```

All code snippets will then be exported to `{SYNTAX_EXPORT_PATH}/{output}`.

## License

This project is licensed under the MIT License.
