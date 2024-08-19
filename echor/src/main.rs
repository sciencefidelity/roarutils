use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Matt Cook <matt@mattcook.dev")
        .about("Rust echo")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .help("Do not print newline")
                .num_args(0)
                .short('n'),
        )
        .get_matches();

    let text = matches
        .get_many::<String>("text")
        .unwrap()
        .map(String::to_owned)
        .collect::<Vec<_>>();
    let omit_newline = matches.get_one::<bool>("omit_newline").unwrap();
    print!(
        "{}{}",
        text.join(" "),
        if *omit_newline { "" } else { "\n" }
    );
}
