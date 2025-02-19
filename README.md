# Code Snip

This crate simplifies the process of creating code snippets, simply pass 2 arguments:
- input; The code you want a snippet for
- output; The output file name

## Environment variables

You can configure the export path via the "SYNTAX_EXPORT_PATH" environment variable, then all code snippets will be exported to `{SYNTAX_EXPORT_PATH}/{output}`
