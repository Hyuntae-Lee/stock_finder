extern crate hyper;
extern crate encoding;
extern crate html5ever;

mod value_getter;
mod csv_hdl;

use csv_hdl::Company;

fn main() {
    // get code list
    let mut company_list : Vec<Company> = Vec::new();
    if csv_hdl::get_company_list("list.csv", &mut company_list) == 0 {
        println!("Cannot get the code list!");
    }

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
