use std::fs::File;
use std::io::Read;

pub struct Company<'a> {
    name : &'a str,
    code : &'a str,
    category : &'a str,
    product : &'a str,
}

// public methods
pub fn get_company_list(path : &str, company_list : &mut Vec<Company>) -> usize {
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

    // parse
    let line_list : Vec<&str> = buff.split("\r\n").collect();
    for item in line_list.clone() {
        let value_list : Vec<&str> = item.split(',').collect();
        company_list.push(
            Company {
                name : value_list[0],
                code : value_list[1],
                category : value_list[2],
                product : value_list[3]
            }
        );
    }

    line_list.len()
}
