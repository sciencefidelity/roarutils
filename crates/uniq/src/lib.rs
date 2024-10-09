use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

use anyhow::{anyhow, Result};
use clap::{Arg, ArgAction, Command};
use indoc::indoc;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Args {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

/// # Errors
///
/// Will return `Err` if `Read` of `Write` operations fail.
pub fn run(args: &Args) -> Result<()> {
    let mut file = open(&args.in_file).map_err(|e| anyhow!("{}: {e}", args.in_file))?;

    let mut out_file: Box<dyn Write> = match &args.out_file {
        Some(out_name) => Box::new(File::create(out_name)?),
        _ => Box::new(io::stdout()),
    };

    let mut print = |num: u64, text: &str| -> Result<()> {
        if num > 0 {
            if args.count {
                write!(out_file, "{num:>7} {text}")?;
            } else {
                write!(out_file, "{text}")?;
            }
        };
        Ok(())
    };

    let (mut line, mut previous) = (String::new(), String::new());
    let mut count: u64 = 0;
    loop {
        let bytes = file.read_line(&mut line)?;
        if bytes == 0 {
            break;
        }

        if line.trim_end() != previous.trim_end() {
            print(count, &previous)?;
            previous.clone_from(&line);
            count = 0;
        }
        count += 1;
        line.clear();
    }
    print(count, &previous)?;
    if !previous.is_empty() && !previous.ends_with('\n') {
        writeln!(out_file)?;
    }

    Ok(())
}

// arg FILE has a default value so cannot panic.
#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn get_args() -> Args {
    let matches = Command::new("uniq")
        .version("0.1.0")
        .author("Matt Cook <matt@mattcook.dev")
        .about(indoc! {"
            Filter adjacent matching lines from INPUT (or standard input),
            writing to OUTPUT (or standard output).

            With no options, matching lines are merged to the first occurrence.
            
            Manditory arguments to long options are manditory for short options too.
        "})
        .help_template(indoc! {"
            Usage: {usage}
            {about}
            {options}

            A field is a run of blanks (usually spaces and/or TABs), then non-blank
            characters. Fields are skipped before chars.

            'uniq' does not detect repeated lines unless they are adjacent.
            You may want to sort the input first, or use 'sort -u' without 'uniq'.

            GNU coreutils online help: <https://www.gnu.org/software/coreutils/>
            Full documentation <https://www.gnu.org/software/coreutils/wc>
            or available locally via: info '(coreutils) wc invocation'
        "})
        .arg(
            Arg::new("in_file")
                .value_name("FILE")
                .help("Input file")
                .default_value("-"),
        )
        .arg(Arg::new("out_file").value_name("FILE").help("Output file"))
        .arg(
            Arg::new("count")
                .value_name("COUNT")
                .help("prefix lines by the number of occurrences")
                .short('c')
                .long("count")
                .action(ArgAction::SetTrue)
                .num_args(0),
        )
        .get_matches();

    let in_file: String = matches
        .get_one("in_file")
        .cloned()
        .expect("in_file not specified");
    let out_file: Option<String> = matches.get_one("out_file").cloned();

    Args {
        in_file,
        out_file,
        count: matches.get_flag("count"),
    }
}

/// # Errors
///
/// Will return `Err` if file fails to open.
pub fn open(filename: &str) -> Result<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
