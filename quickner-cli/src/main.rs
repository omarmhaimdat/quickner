// quickner
//
// NER tool for quick and simple NER annotation
// Copyright (C) 2023, Omar MHAIMDAT
//
// Licensed under Mozilla Public License 2.0
//

mod cli;
mod config;
mod models;
mod utils;

use cli::Cli;
fn main() {
    let cli = <Cli as clap::Parser>::parse();
    cli.process();
}
