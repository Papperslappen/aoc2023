use pom::parser::*;
use std::str::{self};

pub fn space<'a>() -> Parser<'a, u8, ()> {
    one_of(b" \t\r\n").repeat(0..).discard()
}

pub fn posint<'a>() -> Parser<'a, u8, u32> {
    let integer = (one_of(b"123456789") - one_of(b"0123456789").repeat(0..)) | sym(b'0');
    integer
        .collect()
        .convert(str::from_utf8)
        .map(|str| str.parse::<u32>().unwrap())
}
