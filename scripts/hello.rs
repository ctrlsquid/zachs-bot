#!/usr/bin/env -S cargo +nightly -Zscript
// A simple script that uses clap to parse arguments and print a greeting

//! ```cargo
//! [dependencies]
//! clap = { version = "4.2", features = ["derive"] }
//! ```

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version)]
struct Args {
    #[clap(short, long, help = "A name of someone who deserves to be greeted")]
    name: Option<std::path::PathBuf>,
}

fn main() {
    let args = Args::parse();
    match args.name {
        Some(name) => println!("Hello {}", name.to_string_lossy()),
        None => println!("Hello world"),
    }
}