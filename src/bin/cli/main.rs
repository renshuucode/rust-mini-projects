use clap::Parser;

// usage: cargo run --bin cli -- --name Alice --count 3

#[derive(Parser, Debug)] 
struct Cli {
    #[arg(short, long)]
    name: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Cli::parse();
    for _ in 0..args.count {
        println!("Hello, {}!", args.name);
    }
}
