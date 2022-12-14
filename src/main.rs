mod proxy;

use anyhow::Result;
use clap::Parser;
use syslog::{BasicLogger, Facility, Formatter3164};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "localhost:8080")]
    address: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize the logger
    let formatter = Formatter3164 {
        facility: Facility::LOG_USER,
        hostname: None,
        process: "httproxy".into(),
        pid: 0,
    };

    let logger = syslog::unix(formatter).expect("could not connect to syslog");
    log::set_boxed_logger(Box::new(BasicLogger::new(logger)))
        .map(|()| log::set_max_level(log::LevelFilter::Info))?;

    // Initialize the proxy
    proxy::server(args.address)?;

    Ok(())
}
