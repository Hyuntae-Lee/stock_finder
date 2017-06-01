use hyper::client::Client;
use html5ever::parse_document;
use html5ever::rcdom::{RcDom};
use html5ever::tendril::TendrilSink;

use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_949;

use std::io::Read;

pub fn get_page_html<'a>(code : &str) -> Result<String, &'a str> {
    let base_url = "http://finance.naver.com/item/main.nhn?code=";
    let client = Client::new();
    let mut buff = Vec::new();

    // url
    let target_url = base_url.to_string() + code;

    // send request to server
    let mut resp = match client.get(&target_url).send() {
        Err(x)  => {
            println!("{:?}", x);
            return Err("Failed to get the response from the server!");
        },
        Ok(x)   => x
    };

    // read html binary
    match resp.read_to_end(&mut buff) {
        Err(x)  => {
            println!("{:?}", x);
            return Err("Failed to read html binary!");
        },
        Ok(_)   => {}
    };

    // encoding 을 utf-8 로 변경
    let html_str = match WINDOWS_949.decode(&buff[..], DecoderTrap::Replace) {
        Err(x) => {
            println!("{:?}", x);
            return Err("Failed to decode html binary!");
        },
        Ok(x) => x
    };

    return Ok(html_str);
}
