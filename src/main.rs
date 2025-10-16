fn main() {
    if let Err(err) = satukitan_rs::cli::run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
