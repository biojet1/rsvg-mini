use prefixes::{Prefix, Prefixes};
// use crate::{Element, Error};
use rxml::RawEvent;

/// Tree-building parser state
pub struct TreeBuilder {
    // next_tag: Option<(Prefix, String, Prefixes, BTreeMap<String, String>)>,
    // Parsing stack
    // stack: Vec<Element>,
    // Namespace set stack by prefix
    prefixes_stack: Vec<Prefixes>,
    // Document root element if finished
    // pub root: Option<Element>,
}

fn main() {
    use rxml::EventRead;
    let doc = b"<?xml version='1.0'?><hello>World!</hello>";
    let mut fp = rxml::FeedParser::default();
    let result = rxml::as_eof_flag(fp.parse_all(&mut &doc[..], true, |ev| {
        println!("got event: {:?}", ev);
    }));
    // true indicates eof
    assert_eq!(result.unwrap(), true);
}
