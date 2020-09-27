// read all bookmarks from a storage and present as a search engine
// alf list # lists all bookmarks
// alf open DuckDuckGo # opens url with name DuckDuckGo
use clap::{App, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod command;
use command::Command;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
struct Bookmark {
    name: String,
    url: String,
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Bookshelf {
    bookmarks: Vec<Bookmark>,
}

impl Bookshelf {
    pub fn from_file(file: &str) -> Bookshelf {
        use std::fs::File;
        use std::io::prelude::*;
        let mut file = File::open(file).unwrap();

        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let bookshelf = toml::from_str(content.as_str()).unwrap();
        bookshelf
    }

    pub fn find_by_name(&self, name: &str) -> Option<&Bookmark> {
        for bookmark in &self.bookmarks {
            if bookmark.name.to_lowercase() == name.to_lowercase() {
                return Some(&bookmark)
            }
        }
        None
    }

    // pub fn find_by_tag(self, tag: &str) -> Option<Vec<&Bookmark>> {
    //     // let vec: &Vec<_>= &self
    //     //     .bookmarks
    //     //     .into_iter()
    //     //     .filter(|x| x.clone().has_tag(tag))
    //     //     .collect();
    //     let mut vec = Vec::new();
    //     for bookmark in &self.bookmarks {
    //         if bookmark.has_tag(tag) {
    //             vec.push(bookmark);
    //         }
    //     }

    //     if vec.len() == 0 {
    //         return None;
    //     }

    //     Some(vec)
    // }
}

impl Bookmark {
    pub fn new(name: &str, url: &str, tags: Vec<String>) -> Bookmark {
        Bookmark {
            name: name.to_string(),
            url: url.to_string(),
            tags,
        }
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.to_string())
    }
}

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
        .subcommand(SubCommand::with_name("list"))
        .subcommand(
            SubCommand::with_name("open").arg(
                Arg::with_name("name")
                    .short("n")
                    .value_name("NAME")
                    .takes_value(true)
                    .required(true)
                    .help("print debug information verbosely"),
            ),
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("open") {
        let x = matches.value_of("name").unwrap();
        let mut map = HashMap::new();
        map.insert("name".to_string(), x.to_string());
        command::Open::with_arguments(map).run();
    } else {
        command::List::with_arguments(HashMap::new()).run();
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
}
