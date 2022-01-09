extern crate shiorust;

use shiorust::message::{self, traits::*, Parser};

#[test]
fn use_from_external() {
    let r = message::Request::parse("GET SHIORI/3.0\r\nReference0: aaa\r\n\r\n").unwrap();
    assert_eq!(r.get_reference(0).unwrap().as_str(), "aaa");
    assert_eq!(r.get_header("Reference0").unwrap().as_str(), "aaa");
}
