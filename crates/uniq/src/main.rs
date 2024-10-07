fn main() {
    if let Err(e) = uniq::run(&uniq::get_args()) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
