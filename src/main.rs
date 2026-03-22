fn main() {
    let path = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("usage: shirokane <file.sk>");
        std::process::exit(2);
    });

    let source = std::fs::read_to_string(&path).unwrap_or_else(|error| {
        eprintln!("failed to read {path}: {error}");
        std::process::exit(1);
    });

    match shirokane::pipeline::run_source(&source) {
        Ok(output) => println!("{output}"),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
