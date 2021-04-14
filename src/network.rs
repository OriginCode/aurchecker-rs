use std::collections::HashMap;
use anyhow::{Result, anyhow};
use serde::Deserialize;
use reqwest::blocking::Client;
use crate::parser;

const AUR_RPC_LINK: &str = "https://aur.archlinux.org/rpc/?v=5&type=info&arg[]=";

#[derive(Deserialize)]
struct Results {
    Version: String,
}

#[derive(Deserialize)]
struct APIResult {
    results: Vec<Results>,
}

fn fetch_version(pkgname: &str) -> Result<String> {
    let link = format!("{}{}", AUR_RPC_LINK, pkgname.to_string());
    let resp = Client::new().get(&link).send()?;
    let apiresult: APIResult = resp.json()?;
    let results = &apiresult.results;
    let newver: &str;
    if results.len() == 0 {
        return Err(anyhow!("The package has been removed or cannot be accessed."));
    } else {
        newver = &apiresult.results[0].Version;
    }
    Ok(newver.to_string())
}

pub fn fetch_versions(pkglist: &mut HashMap<String, String>) -> Result<HashMap<String, String>> {
    let mut newpkglist = pkglist.clone();
    for (pkgname, pkgver) in pkglist {
        println!("Fetching package {}", &pkgname);
        let newver = fetch_version(pkgname)?;
        if parser::strvercmp(&newver, &pkgver) {
            *newpkglist.get_mut(pkgname).unwrap() = newver;
        }
    }
    Ok(newpkglist)
}