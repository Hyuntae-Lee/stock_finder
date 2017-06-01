extern crate hyper;
extern crate html5ever;
extern crate encoding;

mod lib1;

fn main() {
    // get html string with code
    let html_str = match lib1::get_page_html("063760") {
        Err(x)  => {
            println!("{}", x);
            return;
        },
        Ok(x)   => x
    };

    println!("{}", html_str);
}
