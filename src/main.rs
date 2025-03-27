mod models;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    link: String,
}

fn main() {
    let args = Args::parse();

    println!("Hello {}!", args.link);
}
