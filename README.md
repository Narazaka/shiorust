# shiorust

SHIORI Protocol Parser

## install

```toml
[dependencies]
shiorust = "0.1"
```

## usage

```rust
use shiorust::message::{self, parts::standard_header_name::*, traits::*, Parser};

fn foo() {
    let r = message::Request::parse("GET SHIORI/3.0\r\nReference0: aaa\r\n\r\n").unwrap();
    assert_eq!(r.headers.get_reference(0).unwrap().as_str(), "aaa");
    assert_eq!(r.headers.get("Reference0").unwrap().as_str(), "aaa");
    assert_eq!(r.headers.get(&Sender), r.headers.get_sender());
}
```

## License

[Zlib license](LICENSE)
