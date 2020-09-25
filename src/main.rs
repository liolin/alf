// read all bookmarks from a storage and present as a search engine
// alf list # lists all bookmarks
// alf open DuckDuckGo # opens url with name DuckDuckGo
use serde::{Serialize, Deserialize};
use clap::{Arg, App, SubCommand};
use std::collections::HashMap;

trait Command {
    fn run(self);
    fn with_arguments(args: HashMap<String, String>) -> Self;
}

struct List {
    args: HashMap<String, String>
}

struct Open {
    args: HashMap<String, String>
}

impl Command for List {
    fn run(self) {
        let bookshelf = Bookshelf::from_file("./alf.toml");

        for bookmark in bookshelf.bookmarks {
            println!("{}:\n\t{}", bookmark.name, bookmark.url);
        }
    }

    fn with_arguments(args: HashMap<String, String>) -> Self {
        Self {
            args
        }
    }
}

impl Command for Open {
    fn run(self) {
        let bookshelf = Bookshelf::from_file("./alf.toml");
        if let Some(bookmark) = bookshelf.find_by_name(self.args.get("name").unwrap().as_str()) {
            if webbrowser::open(bookmark.url.as_str()).is_ok() {

            }
        }

    }

    fn with_arguments(args: HashMap<String, String>) -> Self {
        Self {
            args
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Bookmark {
    name: String,
    url: String,
    tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
struct Bookshelf {
    bookmarks: Vec<Bookmark>
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

    pub fn find_by_name(self, name: &str) -> Option<Bookmark> {
        for bookmark in self.bookmarks {
            if bookmark.name.to_lowercase() == name.to_string().to_lowercase() {
                return Some(bookmark)
            }
        }
        None
    }
}

impl Bookmark {
    pub fn new(name: &str, url: &str, tags: Option<Vec<String>>) -> Bookmark {
        Bookmark {
            name: name.to_string(),
            url: url.to_string(),
            tags
        }
    }
}

fn main() {
    let matches = App::new("Alf")
                          .version("0.1.0")
                          .author("Olivier Lischer <olivier.lischer@liolin.ch>")
                          .about("Access your bookmarks from the CLI")
                          .arg(Arg::with_name("store")
                               .short("s")
                               .value_name("STORE")
                               .takes_value(true))
                          .subcommand(SubCommand::with_name("list"))
                          .subcommand(SubCommand::with_name("open")
                                      .arg(Arg::with_name("name")
                                           .short("n")
                                           .value_name("NAME")
                                           .takes_value(true)
                                           .required(true)
                                           .help("print debug information verbosely"))).get_matches();

    if let Some(matches) = matches.subcommand_matches("open") {
        let x = matches.value_of("name").unwrap();
        let mut map = HashMap::new();
        map.insert("name".to_string(), x.to_string());
        Open::with_arguments(map).run();
    } else {
        List::with_arguments(HashMap::new()).run();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use rand::prelude::*;

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
    { name = "DuckDuckGo", url = "https://www.duckduckgo.com" },
    { name = "Google", url = "https://www.google.com", tags = ["search engine"] }
]
"#;
        let (_, path) = create_tmp_file(toml_string);

        let lhs = Bookshelf {
            bookmarks: vec![
                Bookmark {
                    name: "DuckDuckGo".to_string(),
                    url: "https://www.duckduckgo.com".to_string(),
                    tags: None,
                },
                Bookmark {
                    name: "Google".to_string(),
                    url: "https://www.google.com".to_string(),
                    tags: Some(vec!["search engine".to_string()]),
                },
            ]
        };
        let rhs = Bookshelf::from_file(path.as_str());
        assert_eq!(lhs, rhs)
    }
}
