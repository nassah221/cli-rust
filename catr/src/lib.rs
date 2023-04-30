use std::{
    fmt::format,
    fs,
    io::{BufRead, BufReader},
};

use clap::{Arg, ArgAction, Command};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Muhammad Hassan")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .help("Input file(s)")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("number_lines")
                .short('n')
                .help("Number lines")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("number_nonblank_lines")
                .short('b')
                .help("Number nonblank lines")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_lines"),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .map(|s| s.to_owned())
            .collect(),
        number_lines: matches.get_flag("number_lines"),
        number_nonblank_lines: matches.get_flag("number_nonblank_lines"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for file in &config.files {
        match open(file) {
            Err(e) => eprintln!("Failed to open {}: {}", file, e),
            Ok(buf) => {
                let (mut line_number, mut last_num) = (0, 0);
                let mut lines = buf.lines().peekable();
                while let Some(Ok(line)) = lines.next() {
                    let formatted_line = if config.number_lines {
                        format!("{:6}\t{}", line_number + 1, line)
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            last_num += 1;
                            format!("{:6}\t{}", last_num, line)
                        } else {
                            format!("")
                        }
                    } else {
                        format!("{}", line)
                    };
                    line_number += 1;

                    if lines.peek().is_none() {
                        print!("{}{}", formatted_line, "");
                    } else {
                        print!("{}{}", formatted_line, "\n");
                    }
                }
            }
        }
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(std::io::stdin()))),
        _ => Ok(Box::new(BufReader::new(fs::File::open(filename)?))),
    }
}
