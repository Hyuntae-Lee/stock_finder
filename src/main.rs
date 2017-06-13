extern crate hyper;
extern crate encoding;
extern crate html5ever;

mod lib1;
mod lib2;

use lib1::Company;
use std::io::{self, Write};

fn main() {
    // get code list
    let mut name_code_list : Vec<(String, String)> = Vec::new();
    if lib2::get_name_code_list("list.csv", &mut name_code_list) == 0 {
        println!("Read csv fail!");
        return;
    }

    // get company info
    let mut raw_list : Vec<Company> = Vec::new();
    if lib1::get_company_list(name_code_list, &mut raw_list, get_list_progress_callback) == 0 {
        println!("Cannot get the code list!");
    }

    // filter companies
    let mut valid_list : Vec<Company> = Vec::new();
    for item in raw_list {
        // 1. pbr, per, roe 중 하나라도 정보가 없으면 아웃.
        if item.roe() <= 0.0 || item.per() <= 0.0 || item.pbr() <= 0.0 {
            continue;
        }
        // 2. pbr 이 1.0 보다 크면 아웃.
        if item.pbr() > 1.0 {
            continue;
        }
        // 3. roe 가 1.1 보다 작으면 아웃.
        if item.roe() < 11.0 {
            continue;
        }

        valid_list.push(item);
    }

    // output
    println!("name,code,roe,per,pbr\r\n");
    for item in valid_list {
        println!("{},{},{},{},{}", item.name(), item.code(), item.roe(), item.per(), item.pbr());
    }
}

fn get_list_progress_callback(done : usize, total : usize) {
    print!("\r[{}/{}]", done, total);
    io::stdout().flush().unwrap();
}
