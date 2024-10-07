use std::error::Error;

use clap::{Arg, ArgAction, Command};
use indoc::indoc;

type UniqResult<T> = Result<T, Box<dyn Error>>;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn run(config: &Config) -> UniqResult<()> {
    println!("{config:?}");
    Ok(())
}

// arg FILE has a default value so cannot panic.
#[allow(clippy::missing_panics_doc)]
#[must_use]
pub fn get_args() -> Config {
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

    let in_file: String = matches.get_one("in_file").cloned().unwrap();
    let out_file: Option<String> = matches.get_one("out_file").cloned();

    Config {
        in_file,
        out_file,
        count: matches.get_flag("count"),
    }
}
