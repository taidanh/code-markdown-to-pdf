use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::env;
use rand::prelude::*;
use rand::distributions::Alphanumeric;
use pandoc::{Pandoc, OutputKind};

struct Args {
    input: String,
    ft: String,
    output: String,
    comment: String,
}

impl Args {
    fn new(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }
        // init new args struct
        let mut new_args = Args {
            input: String::new(),
            ft: String::new(),
            output: String::new(),
            comment: String::from("//"),
        };
        for i in 1..args.len() {
            if args[i].contains("-i") {
                new_args.input = args[i + 1].clone();
                new_args.ft = new_args.input.split(".").last().unwrap().to_string();
            } else if args[i].contains("-o") {
                new_args.output = args[i + 1].clone();
            } else if args[i].contains("-c") {
                new_args.comment = args[i + 1].clone();
            }
        }
        if new_args.input.is_empty() {
            return Err("No input file");
        }
        if new_args.output.is_empty() {
            return Err("No output file");
        }
        Ok(new_args)
    }
}

fn main() {
    let env_args: Vec<String> = env::args().collect();
    let args = Args::new(&env_args)
        .unwrap_or_else(|err| {
            println!("Problem parsing arguments: {}", err);
            std::process::exit(1);
        });
    let file = File::open(&args.input).expect("Unable to open file");
    let temp_file = format!("{}.md", thread_rng().sample_iter(&Alphanumeric).take(10).map(char::from).collect::<String>());
    let temp = File::create(&temp_file).expect("Failed to create temp file");
    parse(&file, &args.ft, &temp, &args.comment);
    convert_file(&temp_file, &args.output);
}

fn parse(file: &File, ft: &str, temp_file: &File, comment: &str) {
    let reader = BufReader::new(file);
    let mut writer = BufWriter::new(temp_file);
    let mut code_block = false;

    for line in reader.lines() {
        let line = line.unwrap();

        if !line.starts_with(comment) && line.trim().len() > 0 {
            if !code_block {
                code_block = true;
                writer.write(format!("\n```{}\n", ft).as_bytes()).unwrap();
            }
            writer.write(format!("{}\n", &line).as_bytes()).unwrap();
            writer.write("\n".as_bytes()).unwrap();

        } else if line.trim().len() > 0 {
            if code_block {
                code_block = false;
                writer.write("\n```\n".as_bytes()).unwrap();
            }
            writer.write(format!("{}\n", parse_line(&line, comment)).as_bytes()).unwrap();
            writer.write("\n".as_bytes()).unwrap();
        }
    }

    if code_block {
        writer.write("\n```\n".as_bytes()).unwrap();
    }
}

fn parse_line(line: &str, comment: &str) -> String {
    if line.starts_with(comment) {
        for c in comment.len()..line.len() {
            if line.chars().nth(c).unwrap() != ' ' {
                return line[c..].to_string();
            }
        }
        "\n".to_string()
    } else {
        line.to_string()
    }
}

fn convert_file(temp_file: &str, output: &str) {
    let mut pandoc = Pandoc::new();
    pandoc.add_input(&temp_file);
    let path = std::path::PathBuf::from(&output);
    pandoc.set_output(OutputKind::File(path));
    pandoc.execute().unwrap();
    std::fs::remove_file(&temp_file).unwrap();
}
