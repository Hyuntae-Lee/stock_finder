extern crate hyper;
extern crate encoding;
extern crate html5ever;

use html5ever::rcdom::{RcDom, Handle, Node, NodeEnum};
use html5ever::tendril::StrTendril;

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

    // parse html
    let dom = match lib1::parse_html(html_str) {
        Err(x)  => {
            println!("{}", x);
            return;
        },
        Ok(x)   => x
    };

    // find text nodes
    let text_nodes = lib1::find_text_node(&dom.document);

    for handle in text_nodes {
        let node = handle.borrow();
        println!("{:?}", node.node);
    }
}
