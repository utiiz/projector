use clap::Parser;
use rust_projector::{opts::Opts, config::Config};

use anyhow::Result;

fn main() -> Result<()>{
    let opts: Config = Opts::parse().try_into()?;
    println!("{:?}", opts);

    return Ok(());
}
