// quickner
//
// NER tool for quick and simple NER annotation
// Copyright (C) 2023, Omar MHAIMDAT
//
// Licensed under Mozilla Public License 2.0
//

use clap::Parser;
use log::{error, info};
use std::path::PathBuf;

use crate::models::Quickner;

#[derive(Parser)]
#[clap(version = "0.0.1-alpha.1", author = "Omar MHAIMDAT")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to the configuration file
    /// Default: ./config.toml
    #[clap(short, long, default_value = "./config.toml")]
    pub config: PathBuf,
}

impl Cli {
    pub fn process(&self) {
        let config_file = self.config.to_str();
        let quick = Quickner::new(config_file).process(true);
        match quick {
            Ok(_) => info!("Done!"),
            Err(e) => error!("Error: {}", e),
        }
    }
}
