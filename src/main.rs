mod add;
mod blob;
mod deflate;
mod init;

use blob::Blob;
use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let blob = Blob::from_path("empty").unwrap();
    println!("{}", blob.hash);
    println!("{:?}", blob.data);
    let matches = App::new("git")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("init")
                .about("Create an empty Git repository or reinitialize an existing one")
                .arg(
                    Arg::with_name("name")
                        .index(1)
                        .multiple(false)
                        .required(false),
                ),
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
        .get_matches();

    match matches.subcommand() {
        ("init", Some(args)) => {
            match init::init(&args.value_of("name")) {
                Ok(path) => println!("Initialized empty repository in {}", path.display()),
                Err(_) => eprint!("Repository already initialized"),
            };
        }
        ("add", Some(args)) => {
            add::add(&args.values_of("files").unwrap().collect());
        }
        _ => {}
    }
}
