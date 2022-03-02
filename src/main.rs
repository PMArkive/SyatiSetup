mod funcs;

use std::*;

fn main() {
    funcs::downloadandunzip(&funcs::createstr("https://cdn.discordapp.com/attachments/886616711925751829/948264229226565662/CodeWarrior.zip"));
    funcs::downloadandunzip(&funcs::createstr("https://cdn.discordapp.com/attachments/886616711925751829/948264229817958420/Kamek.zip"));
    println!("Downloaded!");
}