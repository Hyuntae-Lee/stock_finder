use hyper::client::Client;

use html5ever::parse_document;
use html5ever::rcdom::{RcDom, Handle, NodeEnum};
use html5ever::tendril::TendrilSink;

use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_949;

use std::io::Read;
use std::fmt;

enum ValueItem {
    NONE,
    ROE,
    PER,
    PBR,
}

pub struct Company {
    name : String,
    code : String,
    roe : f32,
    per : f32,
    pbr : f32,
}

// impl of Company
impl fmt::Debug for Company {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}, {}, {}",
            self.name(), self.code(), self.roe(), self.per(), self.pbr)
    }
}

impl Company {
    pub fn new(name : &str, code : &str, roe : f32, per : f32, pbr : f32) -> Company {
        Company {
            name : name.to_string(),
            code : code.to_string(),
            roe : roe,
            per : per,
            pbr : pbr
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn code(&self) -> &str { &self.code }
    pub fn roe(&self) -> f32 { self.roe }
    pub fn per(&self) -> f32 { self.per }
    pub fn pbr(&self) -> f32 { self.pbr }
}

// public methods
pub fn get_values_with_code<'a>(code : &'a str) -> Result<(f32, f32, f32), &str> {
    let mut html_str = String::new();

    // get html string with code
    if get_page_html(code, &mut html_str) == false {
        return Err("Getting html page is failed!");
    }

    // parse html
    let decoder = parse_document(RcDom::default(), Default::default()).from_utf8();
    let mut html_bytes = html_str.as_bytes();
    let dom = match decoder.read_from(&mut html_bytes) {
        Err(_)  => {
            return Err("Parsing html is failed!");
        },
        Ok(x)   => x
    };


    // get values
    let values = match get_value_from_dom(dom) {
        Err(_)  => {
            return Err("get_value_from_dom() - fail!");
        },
        Ok(x)   => x
    };

    Ok(values)
}

fn get_value_from_dom<'a>(dom : RcDom) -> Result<(f32, f32, f32), &'a str> {

    // find text nodes
    let text_node_list = find_text_node(&dom.document);
    if text_node_list.len() == 0 {
        return Err("Failed to collect nodes.");
    }

    // compose text list
    let mut text_list : Vec<String> = Vec::new();
    if collect_text_in_text_nodes(text_node_list, &mut text_list) == 0 {
        return Err("Failed to collect texts.");
    }

    // result
    let values = match get_values_from_text_list(text_list) {
        Err(_)  => {
            return Err("Failed to get values from text list!");
        },
        Ok(x)   => x
    };

    Ok(values)
}

fn get_page_html<'a>(code : &str, html_str : &mut String) -> bool {
    let base_url = "http://finance.naver.com/item/main.nhn?code=";
    let client = Client::new();
    let mut buff = Vec::new();

    // url
    let target_url = base_url.to_string() + code;

    // send request to server
    let mut resp = match client.get(&target_url).send() {
        Err(x)  => {
            println!("{:?}", x);
            return false;
        },
        Ok(x)   => x
    };

    // read html binary
    match resp.read_to_end(&mut buff) {
        Err(x)  => {
            println!("{:?}", x);
            return false;
        },
        Ok(_)   => {}
    };

    // encoding 을 utf-8 로 변경
    let html_src = match WINDOWS_949.decode(&buff[..], DecoderTrap::Replace) {
        Err(x) => {
            println!("{:?}", x);
            return false;
        },
        Ok(x) => x
    };

    html_str.push_str(&html_src);

    true
}

fn get_values_from_text_list<'a>(text_list : Vec<String>) -> Result<(f32, f32, f32), &'a str> {

    let mut roe_list : Vec<f32> = Vec::new();
    let mut per_list : Vec<f32> = Vec::new();
    let mut pbr_list : Vec<f32> = Vec::new();

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

    if roe_list.len() == 0 {
        return Err("No ROE value!");
    }

    if per_list.len() == 0 {
        return Err("No PER value!");
    }

    if pbr_list.len() == 0 {
        return Err("No PBR value!");
    }

    Ok((roe_list[0], per_list[0], pbr_list[0]))
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
