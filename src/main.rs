extern crate hyper;
extern crate encoding;
extern crate html5ever;

use html5ever::rcdom::{RcDom, Handle, Node, NodeEnum};

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
    let handle_root = dom.document;
    //let node = handle_root.borrow();

    let mut queue : Vec<Handle> = Vec::new();
    let mut text_queue : Vec<Handle> = Vec::new();

    queue.push(handle_root);
    while queue.len() != 0 {
        let handle = queue.remove(0);
        let node = handle.borrow();
        match &node.node {
            Text    => {
                text_queue.push(handle.clone());
            },
            _       => {}
        }
    }

    while text_queue.len() != 0 {
        let handle = text_queue.remove(0);
        println();
    }

    //println!("{:?}", node.node);
}
