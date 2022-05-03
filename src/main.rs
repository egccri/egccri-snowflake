mod config;
mod generator;
mod server;

use clap::Parser;

fn main() {
    // generator::snowflake::snowflake_zookeeper_holder::init();

    // let args = Args::parse();
    // println!("{:?}", args);
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    config: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}
