mod blob;
mod commands;
mod deflate;
mod index;

use commands::*;

use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new("git")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
                .about("Create an empty repository")
                .arg(Arg::with_name("name").index(1)),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Add file contents to index")
                .arg(
                    Arg::with_name("files")
                        .index(1)
                        .multiple(true)
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("checkout").arg(Arg::with_name("path").index(1)))
        .subcommand(SubCommand::with_name("reset").arg(Arg::with_name("hard").long("hard")))
        .get_matches();

    match matches.subcommand() {
        ("init", Some(args)) => {
            match init::init(&args.value_of("name")) {
                Ok(path) => println!("Initialized empty repository in {}", path.display()),
                Err(_) => eprint!("Repository already initialized"),
            };
        }
        ("add", Some(args)) => {
            add::add(&args.values_of("files").unwrap().collect()).unwrap();
        }
        ("checkout", Some(args)) => checkout::checkout(&args.value_of("path")),
        ("reset", Some(args)) => {
            if args.is_present("hard") {
                reset::hard()
            } else {
                reset::reset()
            }
        }
        _ => {}
    }
}
