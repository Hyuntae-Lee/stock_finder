use std::io::{Read, Write};
use std::fs::File;
use company::Company;

pub fn write_company_list_to_file(path : &str, list : &Vec<Company>) -> usize {
    // read csv contents
    let mut file = match File::create(path) {
        Err(x)  => {
            println!("{}", x);
            return 0;
        },
        Ok(x)   => x
    };

    let mut buff = String::new();
    buff.push_str("name,code,roe,pbr,per\r\n");
    for item in list {
        let line = format!("{},{},{},{},{}\r\n", item.name(), item.code(), item.roe(), item.pbr(), item.per());
        buff.push_str(&line);
    }

    let size = match file.write(buff.as_bytes()) {
        Err(x)  => {
            println!("{}", x);
            return 0;
        },
        Ok(x)   => x
    };

    size
}

pub fn get_name_code_list(path : &str, list : &mut Vec<(String, String)>) -> usize {
    // read csv contents
    let mut file = match File::open(path) {
        Err(x)  => {
            println!("{}", x);
            return 0;
        },
        Ok(x)   => x
    };

    let mut buff = String::new();
    let contents_len = match file.read_to_string(&mut buff) {
        Err(x)  => {
            println!("{}", x);
            return 0;
        },
        Ok(x)   => x
    };
    if contents_len == 0 {
        println!("Empty file!");
        return 0;
    }

    // parse
    let line_list : Vec<&str> = buff.split("\r\n").collect();
    if line_list.len() == 0 {
        println!("List file parsing failure!!\r\n");
        return 0;
    }

    for line in line_list {
        let item_list : Vec<&str> = line.split(',').collect();
        if item_list.len() < 2 {
            continue;
        }

        list.push((item_list[0].to_string(), item_list[1].to_string()));
    }

    list.len()
}
