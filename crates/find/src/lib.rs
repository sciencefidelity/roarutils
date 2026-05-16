#![allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]

use anyhow::Result;
use clap::{Arg, ArgAction, Command, ValueEnum, builder::PossibleValue, value_parser};
use indoc::indoc;
use regex::Regex;
use walkdir::{DirEntry, WalkDir};

#[allow(unused)]
#[derive(Debug)]
pub struct Args {
    paths: Vec<String>,
    names: Vec<Regex>,
    entry_types: Vec<EntryType>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum EntryType {
    Dir,
    File,
    Link,
}

impl ValueEnum for EntryType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Dir, Self::File, Self::Link]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Self::Dir => PossibleValue::new("d"),
            Self::File => PossibleValue::new("f"),
            Self::Link => PossibleValue::new("l"),
        })
    }
}

pub fn run(args: &Args) -> Result<()> {
    let type_filter = |entry: &DirEntry| {
        args.entry_types.is_empty()
            || args.entry_types.iter().any(|entry_type| match entry_type {
                EntryType::Link => entry.file_type().is_symlink(),
                EntryType::Dir => entry.file_type().is_dir(),
                EntryType::File => entry.file_type().is_file(),
            })
    };

    let name_filter = |entry: &DirEntry| {
        args.names.is_empty()
            || args
                .names
                .iter()
                .any(|re| re.is_match(&entry.file_name().to_string_lossy()))
    };

    for path in &args.paths {
        let entries = WalkDir::new(path)
            .into_iter()
            .filter_map(|e| match e {
                Err(e) => {
                    eprintln!("{e}");
                    None
                }
                Ok(entry) => Some(entry),
            })
            .filter(type_filter)
            .filter(name_filter)
            .map(|entry| entry.path().display().to_string())
            .collect::<Vec<_>>();

        println!("{}", entries.join("\n"));
    }

    Ok(())
}

pub fn get_args() -> Args {
    let matches = Command::new("find")
        .version("0.1.0")
        .author("Matt Cook <matt@mattcook.dev")
        .about(indoc! {"
            Default path is the current directory; default expression in -print.
            Expression may consist of: operators, options, tests, and actions.

            Operators (decreasing precedence; -and is implicit where no others are given):
                  ( EXPR )   ! EXPR   -not EXPR   EXOR1 -a EXPR2   EXPR1 -and EXPR2
                  EXPR1 -o EXPR2   EXPR1 -or EXPR2   EXPR1 , EXPR2
        "})
        .help_template(indoc! {"
            Usage: {usage}
            {about}
            {options}

            Valid arguments for -D:
            exec, opt, rates, search, stat, time, tree, all, help
            Use '-D help' for a description of the options, or see find(1)

            Please see also the documentation at https://www.gnu.org/software/findutils/.
            You can report (and track progress on fixing) bugs in the \"find\"
            program via the GNU findutils bug-reporting page at
            https://savannah.gnu.org/bugs/?group=findutils or, if
            you have no web access, by sending email to <bug-findutils@gnu.org>.
        "})
        .arg(
            Arg::new("paths")
                .value_name("PATH")
                .help("Search paths")
                .default_value(".")
                .num_args(0..),
        )
        .arg(
            Arg::new("names")
                .value_name("NAME")
                .short('n')
                .long("name")
                .help("Name")
                .value_parser(Regex::new)
                .action(ArgAction::Append)
                .num_args(0..),
        )
        .arg(
            Arg::new("types")
                .value_name("TYPE")
                .short('t')
                .long("type")
                .help("Entry type")
                .value_parser(value_parser!(EntryType))
                .action(ArgAction::Append)
                .num_args(0..),
        )
        .get_matches();

    Args {
        paths: matches
            .get_many("paths")
            .expect("paths required")
            .cloned()
            .collect(),
        names: matches
            .get_many("names")
            .unwrap_or_default()
            .cloned()
            .collect(),
        entry_types: matches
            .get_many("types")
            .unwrap_or_default()
            .cloned()
            .collect(),
    }
}
