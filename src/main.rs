use std::path::PathBuf;

use clap::{arg, Command};

fn cli() -> Command {
    Command::new("rusty")
        .about("A CLI video streaming crash test")
        .subcommand_required(true)
        .subcommand(
            Command::new("start")
                .about("Start video server")
                .arg(arg!(<PATH> "Stuff to add").value_parser(clap::value_parser!(PathBuf))),
        )
        .subcommand(Command::new("stop").about("Stop the server"))
}

fn main() {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("start", sub_matches)) => {
            let path = sub_matches.get_one::<PathBuf>("PATH").expect("required");

            // Convert the PathBuf to a String, if possible
            if let Some(s) = path.to_str() {
                let s = s.to_string();
                println!("Streaming {}", s);
            } else {
                println!("Path is not valid UTF-8");
            }
        }
        Some(("stop", _)) => {
            println!("Stopping")
        }
        _ => unreachable!(),
    }
}
