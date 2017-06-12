extern crate hyper;
extern crate encoding;
extern crate html5ever;

mod lib1;

use lib1::Company;

fn main() {
    // get company list
    let mut raw_list : Vec<Company> = Vec::new();
    if lib1::get_company_list("list.csv", &mut raw_list, get_list_progress_callback) == 0 {
        println!("Cannot get the code list!");
    }

    // filter companies
    let mut valid_list : Vec<Company> = Vec::new();
    for item in raw_list {
        // 1. pbr, per, roe 중 하나라도 정보가 없으면, 아웃.
        if item.roe().len() == 0 || item.per().len() == 0 || item.pbr().len() == 0 {
            continue;
        }
        // 2. pbr 이 1.0 보다 크면 아웃.
        if item.pbr()[0] > 1.0 {
            continue;
        }
        // 3. roe 가 1.5 보다 작으면 아웃.
        if item.roe()[0] < 11.0 {
            continue;
        }

        valid_list.push(item);
    }

    // out put
    println!("name,code,roe,per,pbr\r\n");
    for item in valid_list {
        println!("{},{},{},{},{}",
            item.name(), item.code(),
            item.roe()[0], item.per()[0], item.pbr()[0]);
    }
}

fn get_list_progress_callback(done : usize, total : usize) {
    println!("getting company[{}/{}]", done, total);
}
