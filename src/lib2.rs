use std::io::Read;
use std::fs::File;

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
    for line in line_list {
        let item_list : Vec<&str> = line.split(',').collect();
        list.push((item_list[0].to_string(), item_list[1].to_string()));
    }

    list.len()
}
