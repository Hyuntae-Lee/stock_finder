extern crate hyper;
extern crate encoding;
extern crate html5ever;

mod lib1;
mod lib2;

use lib1::Company;
use std::io::{self, Write};
use std::thread;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::time::Duration;

fn main() {
    // get code list
    let mut name_code_list : Vec<(String, String)> = Vec::new();
    if lib2::get_name_code_list("list.csv", &mut name_code_list) == 0 {
        println!("Read csv fail!");
        return;
    }

    // get company info

    let (tx, rx) : (Sender<Company>, Receiver<Company>) = mpsc::channel();
    let tx_1 = tx.clone();

    let name_code_list_len = name_code_list.len();
    let name_code_list_sub = name_code_list.split_off(name_code_list_len / 2);

    thread::spawn(move || {
        let mut company_list : Vec<Company> = Vec::new();
        if lib1::get_company_list(name_code_list, &mut company_list) == 0 {
            println!("Cannot get the code list!");
        }

        for company in company_list {
            tx.send(company).unwrap();
            thread::sleep(Duration::new(1, 0));
        }
    });

    thread::spawn(move || {
        let mut company_list : Vec<Company> = Vec::new();
        if lib1::get_company_list(name_code_list_sub, &mut company_list) == 0 {
            println!("Cannot get the code list!");
        }

        for company in company_list {
            tx_1.send(company).unwrap();
            thread::sleep(Duration::new(1, 0));
        }
    });

    let mut company_list : Vec<Company> = Vec::new();
    let mut cnt = 0;
    for received in rx {
        company_list.push(received);
        cnt = cnt + 1;

        print!("\r[{}/{}]", cnt, name_code_list_len);
        io::stdout().flush().unwrap();
    }

    // filter companies
    let mut valid_list : Vec<Company> = Vec::new();
    for item in company_list {
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
