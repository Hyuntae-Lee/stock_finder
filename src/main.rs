extern crate hyper;

use hyper::client::Client;
use std::io::Read;

fn main() {
    let client = Client::new();

    let mut res = client.get("http://finance.naver.com/item/main.nhn?code=063760").send().unwrap();

    let mut bin_buff = Vec::new();

    res.read_to_end(&mut bin_buff).unwrap();

    println!("{:?}", bin_buff);
}
