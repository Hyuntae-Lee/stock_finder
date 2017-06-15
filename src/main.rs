extern crate hyper;
extern crate encoding;
extern crate html5ever;

mod lib1;
mod lib2;

use std::io::{self, Write};

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

                company_list.push(Company::new(&name, &code, roe, per, pbr));

                print!("\r[{}/{}]", cnt, name_code_list_len);
                io::stdout().flush().unwrap();
            }
        };
    }

    println!("");

    // compose candidate list
    for item in company_list {
        // filtering
        // - pbr, per, roe 중 하나라도 정보가 없으면 아웃.
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

        // print out candidate
        println!("{}, {}, ROE[{}], PER[{}], PBR[{}]",
            item.name(), item.code(), item.roe(), item.per(), item.pbr());
    }
}
