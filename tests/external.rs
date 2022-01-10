extern crate shiorust;

use shiorust::message::{self, parts::standard_header_name::*, traits::*, Parser};

#[test]
fn use_from_external() {
    let r = message::Request::parse("GET SHIORI/3.0\r\nReference0: aaa\r\n\r\n").unwrap();
    assert_eq!(r.headers.get_reference(0).unwrap().as_str(), "aaa");
    assert_eq!(r.headers.get("Reference0").unwrap().as_str(), "aaa");
    assert_eq!(r.headers.get(&Sender), r.headers.get_sender());
}
