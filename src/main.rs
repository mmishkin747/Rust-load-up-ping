fn main() {
    if let Err(e) = rlup::get_args().and_then(rlup::run) {
        eprintln!("{}", e);
        std::process::exit(10);
    }
}
