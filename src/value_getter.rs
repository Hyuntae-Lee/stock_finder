use hyper::client::Client;

use html5ever::parse_document;
use html5ever::rcdom::{RcDom, Handle, NodeEnum};
use html5ever::tendril::TendrilSink;

use encoding::{Encoding, DecoderTrap};
use encoding::all::WINDOWS_949;

use std::io::Read;

enum ValueItem {
    NONE,
    ROE,
    PER,
    PBR,
}

pub fn get_value_with_code<'a>(code : &'a str) ->
    Result<(Vec<f32>, Vec<f32>, Vec<f32>), &'a str> {
    // get html string with code
    let html_str = match get_page_html(code) {
        Err(x)  => {
            return Err(x);
        },
        Ok(x)   => String::from(x)
    };

    // parse html
    let dom = match parse_html(html_str) {
        Err(x)  => {
            return Err(x);
        },
        Ok(x)   => x
    };

    // get values
    let (roe_list, per_list, pbr_list) = match get_value_from_dom(dom) {
        Err(x)  => {
            return Err(x);
        },
        Ok(x)   => x
    };

    Ok((roe_list, per_list, pbr_list))
}

fn get_value_from_dom<'a>(dom : RcDom) ->
    Result<(Vec<f32>, Vec<f32>, Vec<f32>), &'a str> {
    // find text nodes
    let text_node_list = find_text_node(&dom.document);
    if text_node_list.len() == 0 {
        return Err("Failed to collect nodes.");
    }

    // compose text list
    let text_list = collect_text_in_text_nodes(text_node_list);
    if text_list.len() == 0 {
        return Err("Failed to collect texts.");
    }

    // get values
    Ok(get_values_from_text_list(text_list))
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

fn get_values_from_text_list(text_list : Vec<String>) ->
    (Vec<f32>, Vec<f32>, Vec<f32>) {
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

    (roe_list, per_list, pbr_list)
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

fn collect_text_in_text_nodes(node_list : Vec<Handle>) -> Vec<String> {
    let mut text_list : Vec<String> = Vec::new();

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

    text_list
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
