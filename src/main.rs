fn main() {
    if let Err(e) = nts::run() {
        eprintln!("{}", e);
    }
}
