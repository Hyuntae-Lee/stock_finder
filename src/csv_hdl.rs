use std::fs::File;
use std::io::Read;

pub struct Company {
    name : String,
    code : String,
    category : String,
    product : String,
}

// impl of Val
impl Company {
    pub fn name(&self) -> &str { &self.name }
    pub fn code(&self) -> &str { &self.code }
    pub fn category(&self) -> &str { &self.category }
    pub fn product(&self) -> &str { &self.product }
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
    if contents_len == 0 {
        println!("Empty file!");
        return 0;
    }

    // parse
    let line_list : Vec<&str> = buff.split("\r\n").collect();
    for item in line_list {
        let value_list : Vec<&str> = item.split(',').collect();
        company_list.push(
            Company {
                name : value_list[0].to_string(),
                code : value_list[1].to_string(),
                category : value_list[2].to_string(),
                product : value_list[3].to_string(),
            }
        );
    }

    company_list.len()
}
