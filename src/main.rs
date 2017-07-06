extern crate hyper;
extern crate encoding;
extern crate html5ever;

mod lib1;
mod lib2;

use std::io::{Write, stdout};
use lib1::Company;

fn main() {
    let mut name_code_list : Vec<(String, String)> = Vec::new();
    let mut company_list : Vec<Company> = Vec::new();

    // get code list
    if lib2::get_name_code_list("list.csv", &mut name_code_list) == 0 {
        println!("Read csv fail!");
        return;
    }

    // get company info
    let name_code_list_len = name_code_list.len();
    let mut cnt = 0;
    for (name, code) in name_code_list {
        match lib1::get_values_with_code(&code) {
            Err(_)  => {
                cnt = cnt + 1;
            },
            Ok((roe, per, pbr))   => {
                cnt = cnt + 1;

                // filter
                if roe > 11.0 && per > 0.0 && pbr > 0.0 && pbr < 1.0 {
                    company_list.push(Company::new(&name, &code, roe, per, pbr));
                }

                print!("\r[{}/{}]", cnt, name_code_list_len);
                stdout().flush().unwrap();
            }
        };
    }

    // write result to file
    lib2::write_company_list_to_file("output.csv", &company_list);
}
