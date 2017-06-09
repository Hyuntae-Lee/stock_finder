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

    // find text nodes
    let text_node_list = lib1::find_text_node(&dom.document);

    // compose text list
    let text_list = lib1::collect_text_in_text_nodes(text_node_list);

    // print output - debug
    for text in text_list {
        println!("{}", text);
    }
}
