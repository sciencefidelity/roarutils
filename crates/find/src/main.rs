fn main() {
    if let Err(e) = find::run(&find::get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
