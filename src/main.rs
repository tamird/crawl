use std::collections::HashSet;

#[derive(PartialEq,Eq,Hash)]
struct URI;

fn get_links(uri: &URI) -> HashSet<URI> {
    HashSet::new()
}

fn crawl(root_uri: URI) -> HashSet<URI> {
    let mut visited = HashSet::new();
    let mut frontier = Vec::new();
    frontier.push(root_uri);

    while let Some(uri) = frontier.pop() {
        let links = get_links(&uri);
        visited.insert(uri);
        for link in links {
            if !visited.contains(&link) { frontier.push(link) }
        }
    }

    visited
}

fn main() {
    crawl(URI);
}
