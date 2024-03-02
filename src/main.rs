mod funcs;

use std::io::*;
use std::fs;
use std::path::Path;

static LINKS: &[u8] = include_bytes!("..\\links.txt");

fn main() {
    if Path::new("CodeWarrior").exists() {
        fs::remove_dir_all("CodeWarrior").unwrap();
    }
    if Path::new("Kamek").exists() {
        fs::remove_dir_all("Kamek").unwrap();
    }
    let links = &getlinks();
    for link in links {
        funcs::downloadandunzip(link);
    }
    fs::rename("CodeWarrior-Syati", "CodeWarrior").unwrap();
    fs::rename("Kamek-v1", "Kamek").unwrap();
    println!("Downloaded!")
}

fn getlinks() -> Vec<String> {
    let reader = BufReader::new(LINKS);
    let mut res: Vec<String> = Vec::new();
    for line in reader.lines() {
        res.push(line.unwrap());
    }
    res
}

#[test]
fn test() -> std::result::Result<(), Box<dyn std::error::Error>> {
    fs::rename("CodeWarrior-Syati", "CodeWarrior")?;
    fs::rename("Kamek-v1", "Kamek")?;
    Ok(())
}