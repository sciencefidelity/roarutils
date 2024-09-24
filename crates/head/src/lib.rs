#![allow(dead_code, clippy::missing_errors_doc, clippy::missing_panics_doc)]
use std::io::Result;

use clap::{value_parser, Arg, Command};
use indoc::indoc;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: u64,
    bytes: Option<u64>,
}

pub fn run(config: &Config) -> Result<()> {
    println!("{config:?}");
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
        .arg(
            Arg::new("bytes")
                .short('c')
                .long("bytes")
                .help(indoc! {"
                    print the first NUM bytes of each file;
                      with the leading '-', print all but the last
                      NUM bytes of each file
                "})
                .value_parser(value_parser!(u64).range(1..))
                .conflicts_with("lines"),
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
