use crate::parser;
use anyhow::{anyhow, Result};
use git2::Repository;
use reqwest;
use serde::Deserialize;
use std::{collections::HashMap, path::Path, process::{Command, Stdio}};

const AUR_RPC_URL: &str = "https://aur.archlinux.org/rpc/?v=5&type=info&arg[]=";
const AUR_URL: &str = "https://aur.archlinux.org/";

#[derive(Deserialize)]
struct Results {
    Version: String,
}

#[derive(Deserialize)]
struct APIResult {
    results: Vec<Results>,
}

async fn fetch_version(pkgname: &str) -> Result<String> {
    let url = format!("{}{}", AUR_RPC_URL, pkgname.to_string());
    let resp = reqwest::get(&url).await?;
    let apiresult: APIResult = resp.json().await?;
    let results = &apiresult.results;
    let newver: &str;
    if results.len() == 0 {
        return Err(anyhow!(
            "The package has been removed or cannot be accessed."
        ));
    } else {
        newver = &apiresult.results[0].Version;
    }
    Ok(newver.to_string())
}

async fn fetch_update(pkgname: &str, pkgclone_path: &str) -> Result<()> {
    let pkgpath = format!("{}/{}", pkgclone_path, pkgname);
    let pkgurl = format!("{}{}.git", AUR_URL, pkgname);
    if parser::check_existance(&pkgpath) {
        Command::new("git")
                .arg("pull")
                .current_dir(&pkgpath)
                .stdout(Stdio::null())
                .spawn();
    } else {
        Repository::clone(&pkgurl, Path::new(&pkgpath))?;
    }
    Ok(())
}

pub async fn fetch_updates(
    pkglist: &HashMap<String, String>,
    pkgclone_path: &str,
) -> Result<HashMap<String, String>> {
    let mut newpkglist = pkglist.clone();
    for (pkgname, pkgver) in newpkglist.iter() {
        println!("Fetching package {}", &pkgname);
        let newver = fetch_version(&pkgname).await?;
        if parser::strvercmp(&newver, &pkgver) {
            *newpkglist.get_mut(pkgname).unwrap() = newver;
            tokio::spawn( async move {
                fetch_update(pkgname, pkgclone_path).await.unwrap();
            });
        }
    }
    Ok(newpkglist)
}
