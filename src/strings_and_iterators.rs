// TODO: remove this when you're done with your implementation.
// #![allow(unused_variables, dead_code)]

    // check that request path begins with prefix, which can include wild card
pub fn prefix_matches(prefix: &str, request_path: &str) -> bool {
    // split prefix and request path on "/" match arrays
    let prefix_parts: Vec<&str> = prefix.split("/").collect();
    let path_parts: Vec<&str> = request_path.split("/").collect();
    let mut matches: bool = true;

    println!("prefix vec: {:?}", prefix_parts);
    println!("path vec: {:?}", path_parts);

    if prefix_parts.len() > path_parts.len() {
        return false
    }

    for (i, part) in prefix_parts.iter().enumerate() {
        if *part == "*" {
            continue
        }
        if *part != path_parts[i] {
            matches = false;
            break;
        }
    }
    return matches
    
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

#[allow(dead_code)]
fn main() {}
