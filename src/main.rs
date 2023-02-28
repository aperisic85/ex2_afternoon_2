

use std::{iter::once, env::current_dir};

pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {

      
        let prefix_iter = prefix.split('\\');

        let paths = request_path
            .split('\\')
            .map(|p| Some(p))
            .chain(once(None));


        for (prefix, request_path) in prefix_iter.zip(paths){
            match request_path {
                Some(request_path) => {
                    if (prefix != "*") && (prefix != request_path) {return false;}
                }
                None => return false,
            }

        }
        true
}

fn main(){

    let curent = current_dir().unwrap();
    //println!("path is: {} ", curent.to_str().unwrap());

    let test_string = r#"C:\Users\*\Development\rustrepo\google-comprehesive\ex2_afternoon_2"#;
    let wild_string = r#"C:\*\*\*\*\ex2_afternoon_2"#;

    if prefix_matches(test_string, curent.to_str().unwrap()) {
        println!("good");
    }
    else {
        println!("baaad");
    }

}

#[test]
fn test_matches_without_wildcard() {
    assert!(prefix_matches("/v1/publishers", "/v1/publishers"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc-123"));
    assert!(prefix_matches("/v1/publishers", "/v1/publishers/abc/books"));

    assert!(!prefix_matches("/v1/publishers", "/v1"));
    assert!(!prefix_matches("/v1/publishers", "/v1/publishersBooks"));
    assert!(!prefix_matches("/v1/publishers", "/v1/parent/publishers"));
}

#[test]
fn test_matches_with_wildcard() {
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/bar/books"
    ));
    assert!(prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/books/book1"
    ));

    assert!(!prefix_matches("/v1/publishers/*/books", "/v1/publishers"));
    assert!(!prefix_matches(
        "/v1/publishers/*/books",
        "/v1/publishers/foo/booksByAuthor"
    ));
}
