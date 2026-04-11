use shuka::cli;

fn main() -> Result<(), shuka::error::ShukaError> {
    println!("shuka greetings you\n");
    cli::run()
}
