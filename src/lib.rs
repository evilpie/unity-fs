#[macro_use]
extern crate nom;
extern crate lz4;

use nom::*;
use lz4::Decoder;

use std::str;
use std::str::FromStr;
use std::io::Read;
use std::io::Write;
use std::fs::File;


#[test]
fn emoji() {
  let d = include_bytes!("../emojiassets");
  decode_fs(d);
}

#[derive(Debug,PartialEq,Eq)]
pub struct Header<'a> {
    pub signature:  &'a str,
    pub stream_version: u32,
    pub unity_version:  &'a str,
    pub unity_revision:  &'a str,
    pub file_size: u64,
    pub compressed_header_size: u32,
    pub header_size: u32,
    pub flags: u32
}

fn parse_header(d: &[u8]) -> IResult<&[u8], Header> {
    do_parse!(d,
        signature: map_res!(take_until_and_consume!("\0"), str::from_utf8) >>
        stream_version: be_u32 >>
        unity_version: map_res!(take_until_and_consume!("\0"), str::from_utf8) >>
        unity_revision: map_res!(take_until_and_consume!("\0"), str::from_utf8) >>
        file_size: be_u64 >>
        compressed_header_size: be_u32 >>
        header_size: be_u32 >>
        flags: be_u32 >>
        (Header {
            signature:  signature,
            stream_version: stream_version,
            unity_version: unity_version,
            unity_revision: unity_revision,
            file_size: file_size,
            compressed_header_size: compressed_header_size,
            header_size: header_size,
            flags: flags,
        })
    )
}

pub fn decode_fs(d: &[u8]) {
    if let IResult::Done(d, o) = parse_header(d) {
        println!("{:?}", o);
        let size = o.compressed_header_size as usize;
        let decoder = Decoder::new(&d[..size]).unwrap();
        let (buffer, result) = decoder.finish();
        print!("{:?}", buffer);
    } else {
        println!("error");
    }
}