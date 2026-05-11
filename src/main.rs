use shuka::cli;

fn main() {
    println!("shuka greetings you\n");

    if let Err(err) = cli::run() {
        eprintln!("Error: {err}");
        std::process::exit(1);
    }
}
