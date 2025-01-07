use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author = "Jalever", version = "0.1.0", about = "A simple CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Greet { name: String },
    Add { x: i32, y: i32 },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match cli.command {
        // This program greets the user with the provided name.
        // Usage: cargo run -- greet <NAME>
        // Example: cargo run -- greet "Rustacean"
        Commands::Greet { name } => println!("Hello, {}!", name),
        Commands::Add { x, y } => println!("{} + {} = {}", x, y, x + y),
    }

    Ok(())
}
