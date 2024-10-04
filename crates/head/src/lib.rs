#![allow(dead_code, clippy::missing_errors_doc, clippy::missing_panics_doc)]
use std::io::{self, BufRead, BufReader, Read};
use std::{error::Error, fs::File};

use clap::{value_parser, Arg, Command};
use indoc::indoc;

type HeadResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: u64,
    bytes: Option<u64>,
}

pub fn run(config: &Config) -> HeadResult<()> {
    let num_files = config.files.len();

    for (file_num, filename) in config.files.iter().enumerate() {
        match open(filename) {
            Err(_) => {
                eprintln!("head: cannot open '{filename}' for reading: No such file or directory");
            }
            Ok(mut file) => {
                if num_files > 1 {
                    println!("{}==> {filename} <==", if file_num > 0 { "\n" } else { "" });
                }
                if let Some(num_bytes) = config.bytes {
                    let mut handle = file.take(num_bytes);
                    let mut buffer = vec![0; usize::try_from(num_bytes).expect("integer overflow")];
                    let n = handle.read(&mut buffer)?;
                    print!("{}", String::from_utf8_lossy(&buffer[..n]));
                } else {
                    let mut line = String::new();
                    for _ in 0..config.lines {
                        let bytes = file.read_line(&mut line)?;
                        if bytes == 0 {
                            break;
                        }
                        print!("{line}");
                        line.clear();
                    }
                }
            }
        }
    }
    Ok(())
}

#[must_use]
pub fn get_args() -> Config {
    let matches = Command::new("head")
        .version("0.1.0")
        .author("Matt Cook <matt@mattcook.dev>")
        .about(indoc! {"
            Print the first 10 lines of each FILE to standard output.
            With more than one FILE, precede each with a header giving the file name.

            With no FILE, or when FILE is -, read standard input.

            Mandatory arguments to long options are mandatory for short options too.
        "})
        .help_template(indoc! {"
            Usage: {usage}
            {about}
            {options}

            NUM may have a multiplier suffix:
            b 512, kB 1000, K 1024, MB 1000*1000, M 1024*1024,
            GB 1000*1000*1000, G 1024*1024*1024, and so on for T, P, E, Z, Y, R, Q.
            Binary prefixes can be used, too: KiB=K, MiB=M, and so on.

            GNU coreutils online help: <https://www.gnu.org/software/coreutils/>
            Full documentation <https://www.gnu.org/software/coreutils/head>
            or available locally via: info '(coreutils) head invocation'
        "})
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files(s)")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .help(indoc! {"
                    print the first NUM bytes of each file;
                      with the leading '-', print all but the last
                      NUM bytes of each file
                "})
                .value_parser(value_parser!(u64).range(1..))
                .conflicts_with("lines"),
        )
        .arg(
            Arg::new("lines")
                .short('n')
                .long("lines")
                .value_name("LINES")
                .help(indoc! {"
                    Print the first NUM lines instead of the first 10;
                      with the leading '-', print all but the last
                      NUM lines of each file
                "})
                .value_parser(value_parser!(u64).range(1..))
                .default_value("10"),
        )
        .get_matches();

    let files: Vec<String> = matches
        .get_many("files")
        .expect("arg FILE missing")
        .cloned()
        .collect();

    let bytes = matches.get_one("bytes").copied();
    let lines = matches.get_one("lines").copied().expect("lines invalid");

    Config {
        files,
        lines,
        bytes,
    }
}

fn open(filename: &str) -> HeadResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
