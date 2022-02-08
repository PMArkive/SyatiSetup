extern crate reqwest;
extern crate indicatif;
extern crate thread_control;
use reqwest::*;
use indicatif::*;
use thread_control::*;
use std::process::*;
use std::fs::*;
use std::env::*;
use std::*;

pub fn download(arg: &String, last: bool) -> String {
    let mut req = blocking::get(arg).unwrap();
    let num = arg.rfind("/").unwrap();
    let name = &arg[num + 1..];
    let mut out = File::create(name).unwrap();
    let prog = ProgressBar::new(req.content_length().unwrap());
    prog.set_style(ProgressStyle::default_bar()
    .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes}")
    .progress_chars("#>-"));
    let (flag, control) = make_pair();
    let handel = thread::spawn(move || {
        while flag.is_alive() {
            io::copy(&mut req, &mut out).unwrap();
            break;
        }
    });
    while !control.is_done() {
        prog.set_position(fs::metadata(name).unwrap().len());
        if control.is_done() {
            control.stop();
            break;
        }
    }
    handel.join().unwrap();
    prog.set_position(prog.length());
    if last {
        prog.finish();
    }
    return String::from(name);
}

pub fn downloadandunzip(arg: &String, last: bool) {
    let name = download(arg, last);
    let dir = current_dir().unwrap();
    let d = dir.to_str().unwrap();
    let mut dirn = String::from(d);
    dirn.push_str("\\");
    dirn.push_str("unrar.exe");
    let mut proc = Command::new(dirn).arg(&name).spawn().unwrap();
    proc.wait().unwrap();
    fs::remove_file(name).unwrap();
}

pub fn createstr(arg: &str) -> String {
    return String::from(arg);
}