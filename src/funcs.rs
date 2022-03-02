extern crate reqwest;
extern crate zip_extract;
use zip_extract::*;
use reqwest::*;
use std::path::*;
use std::*;
use std::io::*;

pub fn download(arg: &String) -> (String, Cursor<Vec<u8>>) {
    let mut req = blocking::get(arg).unwrap();
    let num = arg.rfind("/").unwrap();
    let name = &arg[num + 1..];
    let mut out: Cursor<Vec<u8>> = Cursor::new(Vec::new());
    io::copy(&mut req, &mut out).unwrap();
    return (String::from(name), out);
}

pub fn downloadandunzip(arg: &String) {
    let num = arg.rfind("/").unwrap();
    let name = &arg[num + 1..];
    println!("Downloading {}", name);
    let pair = download(arg);
    let (name, file) = pair;
    let folder = Path::new(Path::new(&name).file_stem().unwrap());
    extract(file, folder, false).unwrap();
}

pub fn createstr(arg: &str) -> String {
    return String::from(arg);
}