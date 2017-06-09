extern crate hyper;
extern crate encoding;
extern crate html5ever;

mod lib1;

fn main() {

    let (roe_list, per_list, pbr_list) = match lib1::get_value_with_code("063760") {
        Err(x)  => {
            println!("{}", x);
            return;
        },
        Ok(x)   => x
    };

    // print output - debug
    println!("ROE : {:?}", roe_list);
    println!("PER : {:?}", per_list);
    println!("PBR : {:?}", pbr_list);
}
