use hyper::client::Client;

use html5ever::parse_document;
use html5ever::rcdom::{RcDom, Handle, NodeEnum};
use html5ever::tendril::TendrilSink;

use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_949;

use std::io::Read;
use std::fs::File;

enum ValueItem {
    NONE,
    ROE,
    PER,
    PBR,
}

pub struct Company {
    name : String,
    code : String,
    roe : Vec<f32>,
    per : Vec<f32>,
    pbr : Vec<f32>,
}

// impl of Val
impl Company {
    pub fn name(&self) -> &str { &self.name }
    pub fn code(&self) -> &str { &self.code }
    pub fn roe(&self) -> &Vec<f32> { &self.roe }
    pub fn per(&self) -> &Vec<f32> { &self.per }
    pub fn pbr(&self) -> &Vec<f32> { &self.pbr }
}

// public methods
pub fn get_company_list(path : &str, company_list : &mut Vec<Company>,
    progress_cb : fn( done : usize, total : usize )) -> usize {

    let mut name_code_list : Vec<(String, String)> = Vec::new();
    if get_name_code_list(path, &mut name_code_list) == 0 {
        println!("Read list fail!");
        return 0;
    }

    let mut roe_list : Vec<f32> = Vec::new();
    let mut pbr_list : Vec<f32> = Vec::new();
    let mut per_list : Vec<f32> = Vec::new();

    let total_cnt = name_code_list.len();
    let mut cnt = 0;
    for (name, code) in name_code_list {
        if get_value_with_code(&code, &mut roe_list, &mut per_list, &mut pbr_list) == false {
            println!("Failed to get values for code {}", code);
            continue;
        }

        company_list.push(
            Company {
                name : name,
                code : code,
                roe : roe_list.clone(),
                per : per_list.clone(),
                pbr : pbr_list.clone(),
            }
        );

        progress_cb(cnt, total_cnt);

        cnt = cnt + 1;
    }

    company_list.len()
}

fn get_name_code_list(path : &str, list : &mut Vec<(String, String)>) -> usize {
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

fn get_value_with_code<'a>(code : &'a str, roe_list : &mut Vec<f32>, per_list : &mut Vec<f32>,
    pbr_list : &mut Vec<f32>) -> bool {
    // get html string with code
    let html_str = match get_page_html(code) {
        Err(x)  => {
            println!("{}", x);
            return false;
        },
        Ok(x)   => String::from(x)
    };

    // parse html
    let dom = match parse_html(html_str) {
        Err(x)  => {
            println!("{}", x);
            return false;
        },
        Ok(x)   => x
    };

    // get values
    if get_value_from_dom(dom, roe_list, per_list, pbr_list) == false {
        return false;
    };

    true
}

fn get_value_from_dom<'a>(dom : RcDom, roe_list : &mut Vec<f32>, per_list : &mut Vec<f32>,
    pbr_list : &mut Vec<f32>) -> bool {

    // find text nodes
    let text_node_list = find_text_node(&dom.document);
    if text_node_list.len() == 0 {
        println!("Failed to collect nodes.");
        return false;
    }

    // compose text list
    let mut text_list : Vec<String> = Vec::new();
    if collect_text_in_text_nodes(text_node_list, &mut text_list) == 0 {
        println!("Failed to collect texts.");
        return false;
    }

    //
    get_values_from_text_list(text_list, roe_list, per_list, pbr_list);

    true
}

fn get_page_html<'a>(code : &str) -> Result<String, &'a str> {
    let base_url = "http://finance.naver.com/item/main.nhn?code=";
    let client = Client::new();
    let mut buff = Vec::new();

    // url
    let target_url = base_url.to_string() + code;

    // send request to server
    let mut resp = match client.get(&target_url).send() {
        Err(x)  => {
            println!("{:?}", x);
            return Err("Failed to get the response from the server!");
        },
        Ok(x)   => x
    };

    // read html binary
    match resp.read_to_end(&mut buff) {
        Err(x)  => {
            println!("{:?}", x);
            return Err("Failed to read html binary!");
        },
        Ok(_)   => {}
    };

    // encoding 을 utf-8 로 변경
    let html_str = match WINDOWS_949.decode(&buff[..], DecoderTrap::Replace) {
        Err(x) => {
            println!("{:?}", x);
            return Err("Failed to decode html binary!");
        },
        Ok(x) => x
    };

    return Ok(html_str);
}

fn parse_html<'a>(html_str: String) -> Result<RcDom, &'a str> {
    let mut html_bytes = html_str.as_bytes();
    let parser = parse_document(RcDom::default(), Default::default());
    let decoder = parser.from_utf8();
    let dom = match decoder.read_from(&mut html_bytes) {
        Err(x)  => {
            println!("{}", x);
            return Err("parse_html() fail 1");
        },
        Ok(x)   => x
    };

    return Ok(dom);
}

fn get_values_from_text_list(text_list : Vec<String>,
    roe_list : &mut Vec<f32>, per_list : &mut Vec<f32>, pbr_list : &mut Vec<f32>) {

    let mut item = ValueItem::NONE;
    for text in text_list {
        match item {
            ValueItem::ROE => {
                match text.parse::<f32>() {
                    Ok(x)   => {
                        roe_list.push(x);
                        continue;
                    },
                    Err(_) => {
                        item = text_to_item(&text);
                    }
                };
            },
            ValueItem::PER => {
                match text.parse::<f32>() {
                    Ok(x)   => {
                        per_list.push(x);
                        continue;
                    },
                    Err(_) => {
                        item = text_to_item(&text);
                    }
                };
            },
            ValueItem::PBR => {
                match text.parse::<f32>() {
                    Ok(x)   => {
                        pbr_list.push(x);
                        continue;
                    },
                    Err(_) => {
                        item = text_to_item(&text);
                    }
                };
            },
            _ => item = text_to_item(&text)
        };
    }
}

fn text_to_item(text : &str) -> ValueItem {
    if text == "ROE(%)" {
        return ValueItem::ROE;
    }
    else if text == "PER(%)" {
        return ValueItem::PER;
    }
    else if text == "PBR(배)" {
        return ValueItem::PBR;
    }
    else {
        return ValueItem::NONE;
    }
}

fn collect_text_in_text_nodes(node_list : Vec<Handle>, text_list : &mut Vec<String>) -> usize {

    for handle in node_list {
        let node = handle.borrow();
        match node.node {
            NodeEnum::Text(ref x)   => {
                // get raw data
                let raw_text = format!("{}", x);
                let char_list = raw_text.chars();

                // remove invalid chars
                let mut text = String::new();
                for c in char_list {
                    if c != '\r' && c != '\n' && c != '\t' {
                        text.push(c);
                    }
                }

                // get valid data
                if text.len() == 0 {
                    continue;
                }

                text_list.push(text);
            },
            _                       => {}
        };
    }

    text_list.len()
}

fn find_text_node(root : &Handle) -> Vec<Handle> {
    let mut buffer : Vec<Handle> = Vec::new();

    let node = root.borrow();
    for child in &node.children {
        let child_ref = child.borrow();
        match child_ref.node {
            NodeEnum::Text(_)   => {
                buffer.push(child.clone());
            },
            _                       => {}
        }

        let mut buff_for_child = find_text_node(child);
        buffer.append(&mut buff_for_child);
    }

    buffer
}
