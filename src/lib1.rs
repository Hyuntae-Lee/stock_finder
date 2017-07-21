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

pub fn get_values_with_code<'a>(code : &'a str) -> Result<(f32, f32, f32), String> {
    // get page string
    let html_str = match get_page_html(code) {
        Err(x) => {
            return Err(x);
        },
        Ok(x) => x
    };
    // parse html
    let decoder = parse_document(RcDom::default(), Default::default()).from_utf8();
    let mut html_bytes = html_str.as_bytes();
    let dom = match decoder.read_from(&mut html_bytes) {
        Err(x)  => {
            return Err(format!("{:?}", x));
        },
        Ok(x)   => x
    };

    // get values
    get_value_from_dom(dom)
}

fn get_value_from_dom<'a>(dom : RcDom) -> Result<(f32, f32, f32), String> {
    // find text nodes
    let text_node_list = find_text_node(&dom.document);
    if text_node_list.len() == 0 {
        return Err("Failed to collect nodes.".to_string());
    }

    // compose text list
    let text_list;
    if let Some(x) = collect_text_in_text_nodes(text_node_list) {
        text_list = x;
    }
    else {
        return Err("Failed to collect texts.".to_string());
    }

    // result
    get_values_from_text_list(text_list)
}

fn get_page_html<'a>(code : &str) -> Result<String, String> {
    let base_url = "http://finance.naver.com/item/main.nhn?code=";

    // url
    let target_url = base_url.to_string() + code;

    // send request to server
    let client = Client::new();
    let mut resp = match client.get(&target_url).send() {
        Err(x)  => {
            return Err(format!("{:?}", x));
        },
        Ok(x)   => x
    };

    // read html binary
    let mut buff = Vec::new();
    if let Err(x) = resp.read_to_end(&mut buff) {
        return Err(format!("{:?}", x));
    }

    // encoding 을 utf-8 로 변경해서 반환
    let html_str = match WINDOWS_949.decode(&buff[..], DecoderTrap::Replace) {
        Err(x) => {
            return Err(format!("{:?}", x));
        },
        Ok(x) => x
    };

    Ok(html_str)
}

fn get_values_from_text_list(text_list : Vec<String>) -> Result<(f32, f32, f32), String> {
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
        return Err("No ROE value!".to_string());
    }

    if per_list.len() == 0 {
        return Err("No PER value!".to_string());
    }

    if pbr_list.len() == 0 {
        return Err("No PBR value!".to_string());
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

fn collect_text_in_text_nodes(node_list : Vec<Handle>) -> Option<Vec<String>> {
    let mut text_list : Vec<String> = Vec::new();
    let mut node_iter = node_list.iter();

    while let Some(handle) = node_iter.next() {
        if let NodeEnum::Text(ref x) = handle.borrow().node {
            let text = format!("{}", x)
                        .chars()
                        .filter(|c| *c != '\r' && *c != '\n' && *c != '\t')
                        .collect::<String>();
            if text.len() > 0 {
                text_list.push(text);
            }
        }
    }

    if text_list.len() > 0 {
        Some(text_list)
    }
    else {
        None
    }
}

fn find_text_node(root : &Handle) -> Vec<Handle> {
    let mut buffer : Vec<Handle> = Vec::new();
    let root_bind = root.borrow();
    let mut child_iter = root_bind.children.iter();

    while let Some(child) = child_iter.next() {
        if let NodeEnum::Text(_) = child.borrow().node {
            buffer.push(child.clone());
        }

        let mut buff_for_child = find_text_node(child);
        buffer.append(&mut buff_for_child);
    }

    buffer
}
