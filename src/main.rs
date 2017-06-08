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

    // find text nodes
    find_text_node(&dom.document);
}

fn find_text_node(root : &Handle) -> Vec<Handle> {
    let mut buffer : Vec<Handle> = Vec::new();

    let root_ref = root.borrow();
    for child in &root_ref.children {
        let child_ref = child.borrow();
        match child_ref.node {
            NodeEnum::Text(_)   => {
                buffer.push(child.clone());
            },
            _                       => {}
        }

        let buff_for_child = find_text_node(child);
        buffer.append(buff_for_child);
    }

    buffer
}
