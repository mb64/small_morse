use small_morse::encode;

#[test]
fn letters() {
    for c in b'A'..b'Z' {
        let c = c as char;
        let mut buf: [u8; 1] = [0];
        let s: &str = c.encode_utf8(&mut buf);
        println!("{}: {:#?}", c, encode(s).collect::<Vec<_>>());
    }
}
