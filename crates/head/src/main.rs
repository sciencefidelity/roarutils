fn main() {
    if let Err(e) = head::run(&head::get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
