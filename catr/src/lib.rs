#![allow(dead_code)]
use clap::{Arg, ArgAction, Command};
use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    let mut line_num = 0;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                for line_result in file.lines() {
                    let line = line_result?;
                    if config.number_lines {
                        line_num += 1;
                        println!("{line_num:>6}\t{line}");
                    } else if config.number_nonblank_lines {
                        if !line.is_empty() {
                            line_num += 1;
                            println!("{line_num:>6}\t{line}");
                        } else {
                            println!();
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Matt Cook <matt@mattcook.dev>")
        .about("Rust cat")
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files(s) [default: -]")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("number")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank")
                .short('n')
                .long("number")
                .help("Numbers lines"),
        )
        .arg(
            Arg::new("number_nonblank")
                .action(ArgAction::SetTrue)
                .short('b')
                .long("number-nonblank")
                .help("Number non-blank lines"),
        )
        .get_matches();

    let files: Vec<String> = matches
        .get_many("files")
        .expect("arg FILE missing")
        .cloned()
        .collect();

    Ok(Config {
        files,
        number_lines: matches.get_flag("number"),
        number_nonblank_lines: matches.get_flag("number_nonblank"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
