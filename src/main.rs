mod funcs;

use std::*;
use std::io::*;

static LINKS: &[u8] = include_bytes!("..\\links.txt");

fn main() {
    let links = &getlinks();
    for link in links {
        funcs::downloadandunzip(link);
    }
    println!("Downloaded!");
}

fn getlinks() -> Vec<String> {
    let reader = BufReader::new(LINKS);
    let mut res: Vec<String> = Vec::new();
    for line in reader.lines() {
        res.push(line.unwrap());
    }
    res
}