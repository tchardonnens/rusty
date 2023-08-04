use actix_files::NamedFile;
use actix_web::{web, App, Error, HttpServer};
use clap::{arg, Command};
use std::path::PathBuf;

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

async fn index(path: String) -> Result<NamedFile, Error> {
    let path = PathBuf::from(path);
    Ok(NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let matches = cli().get_matches();
    match matches.subcommand() {
        Some(("start", sub_matches)) => {
            let path = sub_matches.get_one::<PathBuf>("PATH").expect("required");

            // Convert the PathBuf to a String, if possible
            if let Some(s) = path.to_str() {
                let s = s.to_string();
                println!("Streaming {}", s);
                let path_string = s.clone();
                HttpServer::new(move || {
                    let path_clone = path_string.clone();
                    App::new().route("/", web::get().to(move || index(path_clone.clone())))
                })
                .bind("127.0.0.1:8080")?
                .run()
                .await?;
            } else {
                println!("Path is not valid UTF-8");
            }
        }
        Some(("stop", _)) => {
            println!("Stopping")
        }
        _ => unreachable!(),
    }

    Ok(())
}
