fn main() {
    if let Err(e) = wc::run(&wc::get_args()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
