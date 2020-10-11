// read all bookmarks from a storage and present as a search engine
// alf list # lists all bookmarks
// alf open DuckDuckGo # opens url with name DuckDuckGo
use clap::{App, Arg, SubCommand, AppSettings};
use std::boxed::Box;

use alf::command;
use alf::Command;
use alf::Result;

fn main() -> Result {
    let matches = App::new("Alf")
        .version("0.1.0")
        .author("Olivier Lischer <olivier.lischer@liolin.ch>")
        .about("Access your bookmarks from the CLI")
        .setting(AppSettings::SubcommandRequired)
        .arg(
            Arg::with_name("store")
                .short("s")
                .value_name("STORE")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("list").arg(
                Arg::with_name("tag")
                    .short("t")
                    .value_name("TAG")
                    .takes_value(true),
            )

        )
        .subcommand(
            SubCommand::with_name("open").arg(
                Arg::with_name("name")
                    .short("n")
                    .value_name("NAME")
                    .takes_value(true)
                    .required(true),
            ),
        )
        .get_matches();


    let command: Box<dyn Command> = match matches.subcommand() {
        ("open", Some(matches)) => {
            Box::new(command::Open::with_arguments(matches))
        },
        ("list", Some(matches)) => {
            Box::new(command::List::with_arguments(matches))
        }
        ("", None) => {
            return Err(alf::AlfError::NoSubcommand);
        },
        (_, _) => {
            return Err(alf::AlfError::InvalidSubcommand);
        }

    };
    command.run()
}
