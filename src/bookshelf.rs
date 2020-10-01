use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Bookmark {
    pub name: String,
    pub url: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
pub struct Bookshelf {
    pub bookmarks: Vec<Bookmark>,
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

    pub fn find_by_tag(&self, tag: &str) -> Vec<&Bookmark> {
        // let vec: &Vec<_>= &self
        //     .bookmarks
        //     .into_iter()
        //     .filter(|x| x.clone().has_tag(tag))
        //     .collect();
        let mut vec = Vec::new();
        for bookmark in &self.bookmarks {
            if bookmark.has_tag(tag) {
                vec.push(bookmark);
            }
        }
        vec
    }
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
