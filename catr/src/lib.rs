#![allow(dead_code)]
use clap::{Arg, ArgAction, Command};
use indoc::indoc;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
    show_ends: bool,
    show_nonprinting: bool,
    show_tabs: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    let mut line_num = 0;
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {filename}: {err}"),
            Ok(file) => {
                for line_result in file.lines() {
                    let mut line = line_result?;
                    if config.show_ends {
                        line.push('$');
                    }
                    if config.show_tabs {
                        line = line.replace("\t", "^I");
                    }
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
    let matches = Command::new("catr")
        .version("0.1.0")
        .author("Matt Cook <matt@mattcook.dev>")
        .about(indoc! {"
            Concatenate FILE(s) to standard output.

            With no FILE, or when FILE is -, read standard input.",
        })
        .after_help(indoc! {"
            Examples:  
              cat f - g  Output f's contents, then standard input, then g's contents.
              cat        Copy standard input to standard output.
        "})
        .help_template(indoc! {"
            Usage: {usage}
            {about-with-newline}
            {options}

            Examples:  
              cat f - g  Output f's contents, then standard input, then g's contents.
              cat        Copy standard input to standard output.

            GNU coreutils online help: <https://www.gnu.org/software/coreutils/>
            Full documentation <https://www.gnu.org/software/coreutils/cat>
            or available locally via: info '(coreutils) cat invocation'
        "})
        .arg(
            Arg::new("files")
                .value_name("FILE")
                .help("Input files(s)")
                .default_value("-")
                .num_args(1..),
        )
        .arg(
            Arg::new("show_all")
                .action(ArgAction::SetTrue)
                .short('A')
                .long("show-all")
                .help("Equivalent to -vET"),
        )
        .arg(
            Arg::new("number_nonblank")
                .action(ArgAction::SetTrue)
                .short('b')
                .long("number-nonblank")
                .help("Number nonempty output lines"),
        )
        .arg(
            Arg::new("show_nonprinting_ends")
                .action(ArgAction::SetTrue)
                .short('e')
                .help("Equivalent to -vE"),
        )
        .arg(
            Arg::new("show_ends")
                .action(ArgAction::SetTrue)
                .short('E')
                .long("show-ends")
                .help("Display $ at the end of each line"),
        )
        .arg(
            Arg::new("number")
                .action(ArgAction::SetTrue)
                .conflicts_with("number_nonblank")
                .short('n')
                .long("number")
                .help("Numbers all output lines"),
        )
        .arg(
            Arg::new("show_nonprinting_tabs")
                .action(ArgAction::SetTrue)
                .short('t')
                .help("Equivalent to -vT"),
        )
        .arg(
            Arg::new("show_tabs")
                .action(ArgAction::SetTrue)
                .short('T')
                .long("show-tabs")
                .help("Display TAB characters as ^I"),
        )
        .arg(
            Arg::new("ignored")
                .action(ArgAction::SetTrue)
                .short('u')
                .help("(ignored)"),
        )
        .arg(
            Arg::new("show_nonprinting")
                .action(ArgAction::SetTrue)
                .short('v')
                .long("show-nonprinting")
                .help("Use ^ and M- notation, except for LFD and TAB"),
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
        show_ends: matches.get_flag("show_ends"),
        show_nonprinting: matches.get_flag("show_nonprinting"),
        show_tabs: matches.get_flag("show_tabs"),
    })
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
