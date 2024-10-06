use std::io::{self, BufRead, BufReader};
use std::{error::Error, fs::File};

use clap::{Arg, ArgAction, Command};
use indoc::indoc;

type WcResult<T> = Result<T, Box<dyn Error>>;

#[allow(dead_code, clippy::struct_excessive_bools)]
#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

/// # Errors
///
/// Will return `Err` if a file does not exist.
pub fn run(config: &Config) -> WcResult<()> {
    let mut results = Vec::with_capacity(config.files.len());
    let mut tab_size = 0;
    for filename in &config.files {
        match open(filename) {
            Err(_) => eprintln!("wc: {filename}: No such file or directory"),
            Ok(file) => {
                if let Ok(counts) = count(file, filename, config) {
                    tab_size = tab_size.max(counts.2);
                    results.push(counts);
                }
            }
        }
    }
    for result in &results {
        if config.files.len() == 1 && result.1.len() == 1 {
            print!("{}", result.1[0]);
        } else {
            let formatted_figures: Vec<String> =
                result.1.iter().map(|f| format!("{f:>tab_size$}")).collect();
            print!("{}", formatted_figures.join(" "));
        }
        if result.0 == "-" {
            println!();
        } else {
            println!(" {}", result.0);
        }
    }

    if config.files.len() > 1 {
        let totals = results
            .iter()
            .map(|r| r.1.clone())
            .reduce(|a, b| a.iter().zip(b.iter()).map(|(x, y)| x + y).collect())
            .unwrap_or_else(Vec::new);
        let formatted_totals: Vec<String> =
            totals.iter().map(|f| format!("{f:>tab_size$}")).collect();
        print!("{}", formatted_totals.join(" "));
        println!(" total");
    }

    Ok(())
}

/// # Errors
///
/// Will return `Err` if `read_line()` fails.
pub fn count<'a>(
    mut file: impl BufRead,
    filename: &'a str,
    config: &'a Config,
) -> WcResult<(&'a str, Vec<usize>, usize)> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;

        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }
    let mut counts = Vec::with_capacity(4);
    if config.lines {
        counts.push(num_lines);
    }
    if config.words {
        counts.push(num_words);
    }
    if config.bytes {
        counts.push(num_bytes);
    }
    if config.chars {
        counts.push(num_chars);
    }
    let len = count_digits(num_bytes);
    Ok((filename, counts, len))
}

const fn count_digits(n: usize) -> usize {
    if n < 10 {
        1
    } else if n < 100 {
        2
    } else if n < 1000 {
        3
    } else if n < 10_000 {
        4
    } else if n < 100_000 {
        5
    } else if n < 1_000_000 {
        6
    } else if n < 10_000_000 {
        7
    } else if n < 100_000_000 {
        8
    } else if n < 1_000_000_000 {
        9
    } else {
        10
    }
}

// arg FILE has a default value so cannot panic.
#[allow(clippy::missing_panics_doc)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let config = Config {
            files: vec!["test.txt".to_owned()],
            lines: true,
            words: true,
            chars: false,
            bytes: true,
        };
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text), &config.files[0], &config);
        assert!(info.is_ok());
        let expected = ("test.txt", vec![1, 10, 48], 2);
        assert_eq!(info.expect("failed to read file"), expected);
    }
}
