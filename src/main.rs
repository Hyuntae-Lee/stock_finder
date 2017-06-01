extern crate hyper;
extern crate html5ever;
extern crate encoding;

use hyper::client::Client;
use html5ever::parse_document;
use html5ever::rcdom::{RcDom};
use html5ever::tendril::TendrilSink;

use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_949;

use std::io::{Read, Write};
use std::fs::File;

fn main() {
    let mut buff = Vec::new();
    if get_html_binary("063760", &mut buff) == 0 {
        return;
    }

    // encoding 을 utf-8 로 변경
    let decoded = match WINDOWS_949.decode(&buff[..], DecoderTrap::Replace) {
        Err(_) => {
            println!("Failed to decode");
            return;
        },
        Ok(x) => x
    };

    println!("{}", decoded);
}

fn get_html_binary(code : &str, buff : &mut Vec<u8>) -> usize {
    let base_url = "http://finance.naver.com/item/main.nhn?code=";
    let client = Client::new();

    // url
    let target_url = base_url.to_string() + code;

    // send request to server
    let mut resp = match client.get(&target_url).send() {
        Err(x)  => {
            println!("{:?}", x);
            return 0;
        },
        Ok(x)   => x
    };

    // read html binary
    let size = match resp.read_to_end(buff) {
        Err(x)  => {
            println!("{:?}", x);
            return 0;
        },
        Ok(x)   => x
    };

    return size;
}
