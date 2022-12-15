use anyhow::Result;
use log::{error, info};
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{self, Read},
};

pub fn integrity_check() -> Result<()> {
    let mut hasher = Sha256::new();
    let mut binary = File::open("target/release/httproxy")?;
    let mut digest = String::new();
    Read::read_to_string(&mut File::open("httproxy.sha256")?, &mut digest)?;
    io::copy(&mut binary, &mut hasher)?;
    let hash = hex::encode(hasher.finalize());

    if hash != digest.trim() {
        println!("Integrity check failed");
        error!("Integrity check failed");
        std::process::exit(1);
    } else {
        println!("Integrity check passed");
        info!("Integrity check passed");
    }

    Ok(())
}
