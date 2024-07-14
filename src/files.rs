use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

pub fn write_config() {}
/*
pub fn load_config(total_players: &mut u8, ip: &mut String, port: &mut String, nickname: &mut String) {
    let home = dirs::home_dir();
    let file_path: PathBuf = [home.to_str().unwrap(), ".config/monorust.conf"].iter().collect();
    match File::open(&file_path) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            for line in content.lines() {
                // Set info
                if line.starts_with("nickname=") {
                    nickname.push_str(&line[9..]);
                } else if line.starts_with("players=") {
                    let convert = &line[8..];
                    let iconvert: u8 = convert.parse().unwrap();
                    if iconvert >= 2 && iconvert <= 4 {
                        *total_players = 2;
                    } else {
                        *total_players = iconvert;
                    }
                } else if line.starts_with("ip=") {
                    ip.push_str(&line[3..]);
                } else if line.starts_with("port=") {
                    port.push_str(&line[5..]);
                }
            }
        }
        Err(e) => println!("{e}"),
    }
}
*/
