extern crate hyper;
extern crate encoding;
extern crate html5ever;

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

    //
    let doc_node = dom.document;
    let node = doc_node.borrow();

    println!("{:?}", node.node);
}
