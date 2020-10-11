use alf::bookshelf::*;

#[test]
fn test_bookmark_write() {
    let bookmark_without_tags = Bookmark {
        name: "DuckDuckGo".to_string(),
        url: "https://www.duckduckgo.com".to_string(),
        tags: Vec::new(),
    };
    let bookmark_with_one_tag = Bookmark {
        name: "Google".to_string(),
        url: "https://www.google.com".to_string(),
        tags: vec!["search engine".to_string()],
    };
    let bookmark_with_tags = Bookmark {
        name: "Github".to_string(),
        url: "https://www.github.com".to_string(),
        tags: vec!["code".to_string(), "git".to_string()],
    };

    let bytes_without_tags = "DuckDuckGo:\n\thttps://www.duckduckgo.com\n\tTags: empty\n\n".as_bytes();
    let bytes_with_one_tag = "Google:\n\thttps://www.google.com\n\tTags: search engine\n\n".as_bytes();
    let bytes_with_tags = "Github:\n\thttps://www.github.com\n\tTags: code, git\n\n".as_bytes();

    let mut bytes: Vec<u8> = Vec::new();
    bookmark_without_tags.write(&mut bytes).unwrap();
    assert_eq!(String::from_utf8(bytes_without_tags.to_vec()), String::from_utf8(bytes));


    let mut bytes: Vec<u8> = Vec::new();
    bookmark_with_one_tag.write(&mut bytes).unwrap();
    assert_eq!(String::from_utf8(bytes_with_one_tag.to_vec()), String::from_utf8(bytes));

    let mut bytes: Vec<u8> = Vec::new();
    bookmark_with_tags.write(&mut bytes).unwrap();
    assert_eq!(String::from_utf8(bytes_with_tags.to_vec()), String::from_utf8(bytes));
}
