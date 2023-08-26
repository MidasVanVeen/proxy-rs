extern crate clap;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, about, version)]
pub struct Cli {
    #[arg(short, long)]
    pub file: String,
    #[arg(short, long, default_value = "checked-proxies.txt")]
    pub outfile: String,
    #[arg(short, long, default_value = "false")]
    pub verbose: bool,
    #[arg(short, long, default_value = "5")]
    pub timeout: u32,
    #[arg(short, long, default_value = "3")]
    pub retries: u32,
    #[arg(short = 'T', long, default_value = "200")]
    pub threads: u32,
}
