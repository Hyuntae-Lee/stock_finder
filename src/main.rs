extern crate hyper;
extern crate encoding;
extern crate html5ever;

use html5ever::rcdom::{RcDom, Handle, Node, NodeEnum};
use html5ever::tendril::StrTendril;
use std::cell::{RefCell, Ref};

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

    // sort text elements
    let mut queue : Vec<Handle> = Vec::new();
    let mut text_queue : Vec<StrTendril> = Vec::new();

    queue.push(dom.document);
    println!("{}", queue.len());
    for handle in queue {
        let node = handle.borrow();
        match node.node {
            NodeEnum::Text(ref x)    => {
                println!("{}", x);
                text_queue.push((*x).clone());
            },
            _       => {}
        }
    }

    for handle in text_queue {
        let rc_node = &handle;
        println!("{}", rc_node);
    }
}
