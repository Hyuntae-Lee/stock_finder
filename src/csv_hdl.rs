use std::fs::File;
use std::io::Read;

pub struct Company<'a> {
    name : &'a str,
    code : &'a str,
    category : &'a str,
    product : &'a str,
}

// public methods
pub fn get_code_list(path : &str) -> Result<Vec<Company>, &str> {
    let mut company_list : Vec<Company> = Vec::new();

    // read csv contents
    let csv_contents = match read_list_file(path) {
        Err(x)  => {
            println!("{}", x);
            return Err("Geting csv contents error!");
        },
        Ok(x)   => x
    };

    // parse
    let item_list : Vec<&str> = csv_contents.split("\r\n").collect();
    for item in item_list {
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

    Ok(company_list)
}

// static methods
fn read_list_file(csv_path : &str) -> Result<String, &str> {
    let mut file = match File::open(csv_path) {
        Err(x)  => {
            println!("{}", x);
            return Err("File open error!");
        },
        Ok(x)   => x
    };

    let mut csv_contents = String::new();
    let contents_len = match file.read_to_string(&mut csv_contents) {
        Err(x)  => {
            println!("{}", x);
            return Err("File read error!");
        },
        Ok(x)   => x
    };

    if contents_len == 0 {
        return Err("File is empty!");
    }

    Ok(csv_contents)
}
