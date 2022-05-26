```rs
 .S_sSSs      sSSs    sSSs    sSSs_sSSs     .S_sSSs      sSSs   .S_sSSs    
.SS~YS%%b    d%%SP   d%%SP   d%%SP~YS%%b   .SS~YS%%b    d%%SP  .SS~YS%%b   
S%S   `S%b  d%S'    d%S'    d%S'     `S%b  S%S   `S%b  d%S'    S%S   `S%b  
S%S    S%S  S%S     S%S     S%S       S%S  S%S    S%S  S%S     S%S    S%S  
S%S    S&S  S&S     S&S     S&S       S&S  S%S    S&S  S&S     S%S    d*S  
S&S    S&S  S&S_Ss  S&S     S&S       S&S  S&S    S&S  S&S_Ss  S&S   .S*S  
S&S    S&S  S&S~SP  S&S     S&S       S&S  S&S    S&S  S&S~SP  S&S_sdSSS   
S&S    S&S  S&S     S&S     S&S       S&S  S&S    S&S  S&S     S&S~YSY%b   
S*S    d*S  S*b     S*b     S*b       d*S  S*S    d*S  S*b     S*S   `S%b  
S*S   .S*S  S*S.    S*S.    S*S.     .S*S  S*S   .S*S  S*S.    S*S    S%S  
S*S_sdSSS    SSSbs   SSSbs   SSSbs_sdSSS   S*S_sdSSS    SSSbs  S*S    S&S  
SSS~YSSY      YSSP    YSSP    YSSP~YSSY    SSS~YSSY      YSSP  S*S    SSS  
                                                               SP          
                                                               Y
```
Quickly and easily convert a code document to a PDF or MD document

# Installation

## Required:
- [Rust](https://www.rust-lang.org/tools/install)
- [Pandoc](https://pandoc.org/)

## Setup
```sh
git clone https://github.com/taidanh/decoder
cd decoder
make
```

# Usage
```sh
release/decoder -i example.py -o output.pdf -c '#'
```
**Note:** It's important to note that the output filetype matters for how the file is proccessed.

```sh
release/decoder -i example.rs -o output.md
```
This will output a Markdown file

# Features
- ✅ Convert from .md to any pandoc supported conversion
- ✅ Specify any comment string (default: '//')
- ✅ Automatically uses the input filetype to determine code block syntax

### Planned
- ❌ Multiline comments
- ❌ Indented lists
- ❌ Unicode support

# How to style your code

```rs
// # You can type in normal markdown
// ## Just put it after comments
// *You can* `use other` **markdown syntax**

fn main() {
    println!("This code is able to run normally");
}
```
