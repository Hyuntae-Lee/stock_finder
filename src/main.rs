extern crate hyper;
extern crate encoding;
extern crate html5ever;

mod lib1;
mod lib2;

use std::io::{self, Write};
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;

use lib1::Company;

fn main() {
    // get code list
    let mut name_code_list : Vec<(String, String)> = Vec::new();
    if lib2::get_name_code_list("list.csv", &mut name_code_list) == 0 {
        println!("Read csv fail!");
        return;
    }

    // get company info
    // - 연습 삼아 thread 를 사용해 보자
    let (tx, rx) : (Sender<Company>, Receiver<Company>) = mpsc::channel();
    let name_code_list_len = name_code_list.len();
    thread::spawn(move || {
        for (name, code) in name_code_list {
            match lib1::get_values_with_code(&code) {
                Err(_)  => {},
                Ok((roe, per, pbr))   => {
                    let company = Company::new(&name, &code, roe, per, pbr);
                    tx.send(company).unwrap();
                    thread::sleep(Duration::new(1, 0));
                }
            };
        }
    });

    // compose candidate list
    let mut candidate_list : Vec<Company> = Vec::new();
    let mut cnt = 0;
    for item in rx {
        // update screen
        io::stdout().flush().unwrap();

        // progress
        cnt = cnt + 1;
        // clear screen
        print!("\r{}", "                                                           ");
        print!("\r[{}/{}] {}[{}]", cnt, name_code_list_len, item.name(), item.code());

        // filtering
        // - pbr, per, roe 중 하나라도 정보가 없으면 아웃.
        if item.roe() <= 0.0 || item.per() <= 0.0 || item.pbr() <= 0.0 {
            print!(" -> not enough infomation");
            continue;
        }
        // 2. pbr 이 1.0 보다 크면 아웃.
        if item.pbr() > 1.0 {
            print!(" -> too big pbr");
            continue;
        }
        // 3. roe 가 1.1 보다 작으면 아웃.
        if item.roe() < 11.0 {
            print!(" -> too small roe");
            continue;
        }

        candidate_list.push(Company::from(item));
    }

    // print out candidate
    print!("\r");
    for item in candidate_list {
        println!("{:?}", item);
    }
}
