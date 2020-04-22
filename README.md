# minecaft-varint
Minecraft VarInt and VarLong implemetation in Rust, providing minimum memory usage and maximum performance.

[![Crates.io][crates-badge]][crates-url]
[![WTFPL licensed][pl-badge]][pl-url]

[crates-badge]: https://img.shields.io/crates/v/mc-varint.svg
[crates-url]: https://crates.io/crates/mc-varint
[pl-badge]: https://img.shields.io/badge/license-WTFPL-blue.svg
[pl-url]: LICENSE

# Example

## Read a VarInt from a Read

```Rust
use mc_varint::VarIntRead;
use std::io::Cursor;
fn main() {
    // firstly we create a Cursor
    let mut cur = Cursor::new(vec![0xff, 0xff, 0xff, 0xff, 0x07]);
    // secondly we read from it
    let value = cur.read_var_i32().unwrap();
    // the value is 2147483647
    assert_eq!(value, 2147483647);
}
```

## Write a VarInt to a Write

```Rust
use mc_varint::{VarInt, VarIntWrite};
use std::io::Cursor;
fn main() {
    // firstly we create a Cursor and a VarInt
    let mut cur = Cursor::new(Vec::with_capacity(5));
    // secondly we write the VarInt to the Cursor
    cur.write_var_i32(2147483647).unwrap();
    // now the value is written to cur.
    assert_eq!(cur.into_inner(), vec![0xff, 0xff, 0xff, 0xff, 0x07]);
}
```

