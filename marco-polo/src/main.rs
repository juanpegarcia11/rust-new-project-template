/*A Marco Polo CLI that uses Clap and imports lib.rs marco_polo function */

use clap::Parser;
use marco_polo::marco_polo;

/// CLI tool for Marco Polo
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name we pass to Marco Polo function
    #[arg(short, long)]
    name: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", marco_polo(&args.name));
    println!("Hello, world!");
}
