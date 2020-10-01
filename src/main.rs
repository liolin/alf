// read all bookmarks from a storage and present as a search engine
// alf list # lists all bookmarks
// alf open DuckDuckGo # opens url with name DuckDuckGo
use clap::{App, Arg, SubCommand};
use std::collections::HashMap;

use alf::command;
use alf::command::Command;

fn main() {
    let matches = App::new("Alf")
        .version("0.1.0")
        .author("Olivier Lischer <olivier.lischer@liolin.ch>")
        .about("Access your bookmarks from the CLI")
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

    if let Some(matches) = matches.subcommand_matches("open") {
        let x = matches.value_of("name").unwrap();
        let mut map = HashMap::new();
        map.insert("name".to_string(), x.to_string());
        command::Open::with_arguments(map).run();
    } else if let Some(matches) = matches.subcommand_matches("list") {
        let mut map = HashMap::new();
        if let Some(tag) = matches.value_of("tag") {
            map.insert("tag".to_string(), tag.to_string());
        }
        command::List::with_arguments(map).run();
    } else {
        let map = HashMap::new();
        command::List::with_arguments(map).run();
    }
}

