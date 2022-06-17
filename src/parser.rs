use std::fs::File;
use std::io::*;

pub struct Parser {
    indent: usize,
    ft: String,
    comment: String,
    file: File,
    temp_file: File,
}

impl Parser {
    pub fn new(ft: &str, comment: &str, path: &str, temp_path: &str) -> Parser {
        let file = File::open(path).unwrap();
        let temp_file = File::open(temp_path).expect("Parser failed to open temp file.");
        Parser {
            indent: 0,
            ft: ft.to_owned(),
            comment: comment.to_owned(),
            file,
            temp_file,
        }
    }

    pub fn parse(&mut self) {
        let copy = self.file.try_clone().unwrap_or_else(|err| {
            println!("Problem cloning file: {}", err);
            std::process::exit(1);
        });
        let temp_copy = self.file.try_clone().unwrap_or_else(|err| {
            println!("Problem cloning temp file: {}", err);
            std::process::exit(1);
        });
        let reader = BufReader::new(copy);
        let mut writer = BufWriter::new(temp_copy);
        let mut code_block = false;

        for line in reader.lines() {
            let line = line.unwrap();

            if !line.starts_with(&self.comment) && line.trim().len() > 0 {
                if !code_block {
                    code_block = true;
                    writer
                        .write(format!("\n```{}\n", &self.ft).as_bytes())
                        .unwrap();
                    self.indent = 0;
                }
                writer.write(format!("{}\n", &line).as_bytes()).unwrap();
                writer.write("\n".as_bytes()).unwrap();
            } else if line.trim().len() > 0 {
                if code_block {
                    code_block = false;
                    writer.write("\n```\n".as_bytes()).unwrap();
                }
                writer
                    .write(format!("{}\n", self.parse_line(&line)).as_bytes())
                    .unwrap();
                writer.write("\n".as_bytes()).unwrap();
            }
        }
    }

    fn parse_line(self, line: &str) -> String {
        if line.starts_with(&self.comment) {
            let mut c = 0;
            if self.indent == 0 {
                for i in self.comment.len()..line.len() {
                    if line.chars().nth(i).unwrap() != ' ' {
                        c = i;
                    }
                }
            } else {
                c = self.indent;
            }
            line[c..].to_owned()
        } else {
            line.to_owned()
        }
    }
}
