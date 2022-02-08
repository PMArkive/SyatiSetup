use std::env::*;
use std::process::*;

fn main() {
    set_current_dir("unrar").unwrap();
    Command::new("go").arg("build").spawn().unwrap().wait().unwrap();
}