use std::{
    error::Error,
    fs::File,
    io::{self, BufRead, BufReader},
};

use clap::{Arg, ArgAction, Command};
use indoc::indoc;

type WcResult<T> = Result<T, Box<dyn Error>>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

pub fn run(config: &Config) -> WcResult<()> {
    for filename in &config.files {
        match open(filename) {
            Err(_) => eprintln!("wc: {filename}: No such file or directory"),
            Ok(_file) => println!("Opened {filename}"),
        }
    }
    Ok(())
}

#[must_use]
pub fn get_args() -> Config {
    let matches = Command::new("wc")
        .version("0.1.0")
        .author("Matt Cook <matt@mattcook.dev")
        .about(indoc! {"
            Print newline, word, and byte counts for each FILE, and a total line if
            more than one FILE is specified. A word is a nonempty sequence of non white
            space delimited by white space characters or by start or end of input.

            With no FILE, or when FILE is -, read standard input.

            The options below may be used to select which counts are printed, always in
            the following order: newline, word, character, byte, maximum line length.
        "})
        .help_template(indoc! {"
            Usage: {usage}
            {about}
            {options}

            GNU coreutils online help: <https://www.gnu.org/software/coreutils/>
            Full documentation <https://www.gnu.org/software/coreutils/wc>
            or available locally via: info '(coreutils) wc invocation'
        "})
        .arg(
            Arg::new("files")
                .value_name("FILES")
                .help("Input file(s)")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .value_name("BYTES")
                .action(ArgAction::SetTrue)
                .help("print the byte counts"),
        )
        .arg(
            Arg::new("chars")
                .short('m')
                .long("chars")
                .value_name("CHARS")
                .action(ArgAction::SetTrue)
                .help("print the character counts"),
        )
        .arg(
            Arg::new("lines")
                .short('l')
                .long("lines")
                .value_name("LINES")
                .action(ArgAction::SetTrue)
                .help("print the newline counts"),
        )
        .arg(
            Arg::new("words")
                .short('w')
                .long("words")
                .value_name("WORDS")
                .action(ArgAction::SetTrue)
                .help("print the word counts"),
        )
        .get_matches();

    let files: Vec<String> = matches
        .get_many("files")
        .expect("arg FILE missing")
        .cloned()
        .collect();

    let mut lines = matches.get_flag("lines");
    let mut words = matches.get_flag("words");
    let mut bytes = matches.get_flag("bytes");
    let mut chars = matches.get_flag("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        (lines, words, bytes, chars) = (true, true, true, false);
    }

    Config {
        files,
        lines,
        words,
        bytes,
        chars,
    }
}

fn open(filename: &str) -> WcResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
