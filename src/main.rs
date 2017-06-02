extern crate hyper;
extern crate html5ever;
extern crate encoding;

use html5ever::parse_document;
use html5ever::rcdom::{RcDom};
use html5ever::tendril::TendrilSink;

mod lib1;

fn main() {
    // get html string with code
    let html_str = match lib1::get_page_html("063760") {
        Err(x)  => {
            println!("{}", x);
            return;
        },
        Ok(x)   => String::from(x)
    };

    let parser = parse_document(RcDom::default(), Default::default());

    println!("{}", html_str);
}
