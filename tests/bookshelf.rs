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
    let rhs = Bookshelf::from_file(path.as_str()).unwrap();
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
