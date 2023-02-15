use clap::Parser;
use rust_projector::opts::Opts;

fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
