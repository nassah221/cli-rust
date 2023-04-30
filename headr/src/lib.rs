use std::{
    fs,
    io::{BufRead, BufReader, Read},
};

use clap::{Arg, Command};

type MyResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: usize,
    bytes: Option<usize>,
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("headr")
        .version("0.1.0")
        .author("Muhammad Hassan")
        .about("Rust head")
        .arg(
            Arg::new("files")
                .help("Input file(s)")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("lines")
                .help("Count lines")
                .short('n')
                .default_value("10")
                .value_parser(clap::value_parser!(usize)),
        )
        .arg(
            Arg::new("bytes")
                .help("Count bytes")
                .short('c')
                .conflicts_with("lines")
                .value_parser(clap::value_parser!(usize)),
        )
        .get_matches();

    Ok(Config {
        files: matches
            .get_many::<String>("files")
            .unwrap()
            .map(|s| s.to_owned())
            .collect(),
        lines: matches.get_one("lines").copied().unwrap(),
        bytes: matches.get_one("bytes").copied(),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(mut reader) => {
                if let Some(num_bytes) = config.bytes {
                    let mut handle = reader.take(num_bytes as u64);
                    let mut buf = vec![0; num_bytes];
                    let bytes_read = handle.read(&mut buf)?;
                    print!("{}", String::from_utf8_lossy(&buf[..bytes_read]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes_read = reader.read_line(&mut line)?;
                        if bytes_read == 0 {
                            break;
                        }
                        print!("{}", line);
                        line.clear();
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
