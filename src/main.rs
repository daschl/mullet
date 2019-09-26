use actix_rt::System;

mod cluster;
mod config;
mod node;
mod service;
mod state;

use crate::config::MulletClusterConfig;
use cluster::Cluster;
use slog::debug;
use slog::o;
use slog::Drain;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let logger = slog::Logger::root(drain, o!());

    let options = Options::from_args();

    let raw_config = fs::read_to_string(options.config_path).expect("Could not load config file");
    let config: MulletClusterConfig =
        serde_json::from_str(&raw_config).expect("Cannot parse config");
    debug!(logger, "Loaded configuration {:?}", config);

    let system = System::new("mullet");

    let cluster = Cluster::new(config, logger);
    cluster.run();

    system.run().expect("Cannot run actix system");
}

#[derive(Debug, StructOpt)]
struct Options {
    #[structopt(short = "c", long = "config", parse(from_os_str))]
    config_path: PathBuf,
}
