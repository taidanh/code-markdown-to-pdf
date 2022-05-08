# Code Markdown to PDF
Quickly and easily convert a code document to a PDF or MD document

# Installation

## Required:
- [Rust](https://www.rust-lang.org/tools/install)
- [Pandoc](https://pandoc.org/)

## Setup
```sh
git clone https://github.com/taidanh/code-markdown-to-pdf/
cd code-markdown-to-pdf
make
```

# Usage
```sh
release/code-to-md -i example.rs -o output.pdf
```
**Note:** It's important to note that the output filetype matters for how the file is proccessed.

```sh
release/code-to-md -i example.rs -o output.md
```
This will output a Markdown file

# How to style your code

```rs
// # You can type in normal markdown
// ## Just put it after comments
// *You can* `use other`[markdown syntax](https://github.com/taidanh)
```
