mod funcs;

use std::*;

fn main() {
    funcs::download(&funcs::createstr("https://cdn.discordapp.com/attachments/875554071447232573/941148579723092088/unrar.exe"), false);
    funcs::downloadandunzip(&funcs::createstr("https://cdn.discordapp.com/attachments/643925511906656257/829423349574533130/CodeWarrior.rar"), false);
    funcs::downloadandunzip(&funcs::createstr("https://cdn.discordapp.com/attachments/643925511906656257/829424059191525427/Kamek.rar"), true);
    fs::remove_file("unrar.exe").unwrap();
    println!("Downloaded!");
}