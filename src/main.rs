mod cli;
mod config;
mod models;
mod utils;

use cli::Cli;
fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let cli = <Cli as clap::Parser>::parse();
    cli.process();
}
