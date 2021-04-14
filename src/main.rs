#![allow(non_snake_case)]

use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};
use crate::parser::check_existance;

mod network;
mod parser;

const DEFAULT_LIST_PATH: &str = ".config/aurchk/pkgs.json";
const DEFAULT_CLONE_PATH: &str = ".cache/aurchk/";
const CONFIG_PATH: &str = ".config/aurchk/config.json";

#[derive(Serialize, Deserialize, Debug)]
struct Config {
    pkgListPath: String,
    pkgClonePath: String,
}

fn read_conf(path: &str) -> Result<Config> {
    let content = fs::read(path)?;
    let config: Config = serde_json::from_slice(&content)?;
    Ok(config)
}

fn write_default_conf(home: &str, path: &str) -> Result<Config> {
    let config_file = fs::File::create(path)?;
    let list_path = format!("{}/{}", home, DEFAULT_LIST_PATH);
    let clone_path = format!("{}/{}", home, DEFAULT_CLONE_PATH);
    let default_conf = Config {
        pkgListPath: list_path,
        pkgClonePath: clone_path,
    };
    serde_json::to_writer_pretty(config_file, &default_conf)?;
    Ok(default_conf)
}

#[tokio::main]
async fn main() -> Result<()> {
    let home: String;
    if let Some(s) = dirs::home_dir() {
        home = s.into_os_string().into_string().unwrap();
    } else {
        return Err(anyhow!("Home directory not found!"));
    }

    let config_path = format!("{}/{}", home, CONFIG_PATH);
    let config: Config;
    if check_existance(&config_path) {
        config = read_conf(&config_path)?;
    } else {
        config = write_default_conf(&home, &config_path)?;
    }

    let pkglist_path = format!("{}/{}", home, DEFAULT_LIST_PATH);
    let pkglist: HashMap<String, String>;
    if check_existance(&pkglist_path) {
        let pkglist_data = fs::read(&config.pkgListPath)?;
        pkglist = serde_json::from_slice(&pkglist_data)?;
        println!("{:?}", pkglist)
    } else {
        return Err(anyhow!(
            "Package list not found, consider write a list manually."
        ));
    }

    let pkgclone_path = format!("{}/{}", home, DEFAULT_CLONE_PATH);
    if !check_existance(&pkgclone_path) {
        fs::create_dir(Path::new(&pkgclone_path))?;
    }
    let newpkglist = network::fetch_updates(&pkglist, &pkgclone_path).await?;
    let pkglist_file = fs::File::create(&config.pkgListPath)?;
    serde_json::to_writer_pretty(pkglist_file, &newpkglist)?;

    Ok(())
}
