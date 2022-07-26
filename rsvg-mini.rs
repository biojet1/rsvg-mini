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
