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

#[cfg(test)]
mod test {
    use alf::bookshelf::*;
    use rand::prelude::*;
    use std::fs::File;
    use std::io::prelude::*;

    fn create_tmp_file(content: &str) -> (File, String) {
        let mut name = String::new();
        let mut rng = thread_rng();
        let letters_iter = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
        for _ in 0..50 {
            let x = letters_iter.clone();
            name.push(x.choose(&mut rng).unwrap());
        }
        let name = format!("/tmp/alf_test_{}.toml", name);
        let mut file = File::create(name.clone()).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        (file, name)
    }

    #[test]
    fn test_parse_toml_file_to_bookshelf() {
        let toml_string = r#"
bookmarks = [
    { name = "DuckDuckGo", url = "https://www.duckduckgo.com", tags = []},
    { name = "Google", url = "https://www.google.com", tags = ["search engine"] }
]
"#;
        let (_, path) = create_tmp_file(toml_string);

        let lhs = Bookshelf {
            bookmarks: vec![
                Bookmark {
                    name: "DuckDuckGo".to_string(),
                    url: "https://www.duckduckgo.com".to_string(),
                    tags: Vec::new(),
                },
                Bookmark {
                    name: "Google".to_string(),
                    url: "https://www.google.com".to_string(),
                    tags: vec!["search engine".to_string()],
                },
            ],
        };
        let rhs = Bookshelf::from_file(path.as_str());
        assert_eq!(lhs, rhs);
    }

    #[test]
    fn test_find_by_name() {
        let google = Bookmark {
            name: "Google".to_string(),
            url: "https://www.google.com".to_string(),
            tags: vec!["search engine".to_string()],
        };
        let bookshelf = Bookshelf {
            bookmarks: vec![
                Bookmark {
                    name: "DuckDuckGo".to_string(),
                    url: "https://www.duckduckgo.com".to_string(),
                    tags: Vec::new(),
                },
                google.clone()
            ],
        };

        let rhs = bookshelf.find_by_name("Google");
        assert_eq!(Some(&google), rhs);
        let rhs = bookshelf.find_by_name("google");
        assert_eq!(Some(&google.clone()), rhs);
        assert_eq!(None, bookshelf.find_by_name("i am not here"));
    }

    #[test]
    fn test_bookmark_has_tag() {
        let google = Bookmark {
            name: "Google".to_string(),
            url: "https://www.google.com".to_string(),
            tags: vec!["search engine".to_string()],
        };

        let empty = Bookmark {
            name: "Google".to_string(),
            url: "https://www.google.com".to_string(),
            tags: Vec::new(),
        };

        assert_eq!(true, google.has_tag("search engine"));
        assert_eq!(false, google.has_tag("i am not here"));
        assert_eq!(false, empty.has_tag("i am not here"));
    }

    #[test]
    fn test_bookshelf_find_by_tag() {
        let google = Bookmark {
            name: "Google".to_string(),
            url: "https://www.google.com".to_string(),
            tags: vec!["search engine".to_string()],
        };
        let duckduckgo = Bookmark {
            name: "DuckDuckGo".to_string(),
            url: "https://www.duckduckgo.com".to_string(),
            tags: vec!["search engine".to_string()],
        };
        let github = Bookmark {
            name: "Github".to_string(),
            url: "https://www.github.com".to_string(),
            tags: Vec::new(),
        };

        let bookshelf = Bookshelf {
            bookmarks: vec![
                google.clone(),
                duckduckgo.clone(),
                github.clone()
            ],
        };

        let rhs = bookshelf.find_by_tag("search engine");
        assert_eq!(vec![&google, &duckduckgo], rhs);
        let lhs: Vec<&Bookmark> = Vec::new();
        assert_eq!(lhs, bookshelf.find_by_tag("i am not here"));
    }
}
