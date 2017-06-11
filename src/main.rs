extern crate hyper;
extern crate encoding;
extern crate html5ever;

mod value_getter;
mod csv_hdl;

fn main() {
    // get code list
    let code_list = csv_hdl::get_code_list("list.csv");

    // get values
    let (roe_list, per_list, pbr_list) =
        match value_getter::get_value_with_code("063760") {
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
