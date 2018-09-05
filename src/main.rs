extern crate hyper;
extern crate encoding;
extern crate html5ever;

mod company;
mod network_hdlr;
mod file_hdlr;

use std::io::{Write, stdout};
use company::Company;

fn main() {
    let mut name_code_list : Vec<(String, String)> = Vec::new();
    let mut company_list : Vec<Company> = Vec::new();

    println!("Start\r\n");

    // get code list
    if file_hdlr::get_name_code_list("list.csv", &mut name_code_list) == 0 {
        println!("Read csv fail!");
        return;
    }

    // get company info
    let name_code_list_len = name_code_list.len();
    let mut cnt = 0;
    for (name, code) in name_code_list {
        match network_hdlr::get_values_with_code(&code) {
            Err(e)  => {
                cnt = cnt + 1;
                println!("Getting vaule failure!! [{}]", e);
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
    file_hdlr::write_company_list_to_file("output.csv", &company_list);
}
