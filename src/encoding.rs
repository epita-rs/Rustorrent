use std::collections::HashMap;

// TODO recover from overflow
macro_rules! _i64_overflow_check {
        ($args:expr) => {
            {
                let (current, added) = $args;
                if i64::MAX - added < current {
                    panic!("i64 overflow")
                }
                current + added
            }
        }
}

#[derive(Debug, PartialEq)]
pub enum BeNode
{
    NUM(i64),
    STR(String),
    LIST(Vec<BeNode>),
    DICT(HashMap<String, BeNode>),
}

pub fn be_decode(input: String) -> BeNode
{
    let buffer = input.chars().collect::<Vec<char>>();
    be_decode_rec(&buffer, &mut 0)
}

pub fn be_decode_rec(buffer: &Vec<char>, idx: &mut usize) -> BeNode
{
    match buffer[*idx] {
        'i' => be_decode_num(buffer, idx),
        '0'..='9' => be_decode_str(buffer, idx),
        'l' => be_decode_list(buffer, idx),
        'd' => be_decode_dict(buffer, idx),
        _ => panic!("failed to decode the input : incorrect bencoding"),
    }
}

pub fn be_decode_num(buffer: &Vec<char>, idx: &mut usize) -> BeNode
{
    // eat 'i'
    *idx += 1;

    let mut nb:i64  = 0;
    // handle minus
    let sign = if buffer[*idx] == '-' { *idx += 1; -1 } else { 1 };

    while *idx < buffer.len() {
        match buffer[*idx] {
            'e' => break,
                // Atoi ; while digit eat + check overflow
            '0'..='9' => nb = nb * 10 + ((buffer[*idx] as i64) - ('0' as i64)),
            _ => panic!("failed to decode the input: incorrect num bencoding"),
        }
        *idx += 1;
    }
    
    // eat 'e'
    *idx += 1;

    BeNode::NUM(nb * sign)
}

pub fn be_decode_str(buffer: &Vec<char>, idx: &mut usize) -> BeNode
{
    let mut nb:usize = 0;
    while *idx < buffer.len() {
        match buffer[*idx] {
            ':' => break,
            // Atoi gives nb
            '0'..='9' => nb = nb * 10 + ((buffer[*idx] as usize) - ('0' as usize)),
            _ => panic!("failed to decode the input: incorrect str bencoding"),
        }
        *idx += 1;
    }

    // eat ':'
    *idx += 1;
    let value:String = buffer[*idx..(*idx + nb)].iter().collect::<String>();
    // eat nb
    *idx += nb;

    BeNode::STR(value)
}

pub fn be_decode_list(buffer: &Vec<char>, idx: &mut usize) -> BeNode
{
    // eat 'l'
    *idx += 1;

    let mut list:Vec<BeNode> = vec![];
    while *idx < buffer.len() {
        match buffer[*idx] {
            'e' => break,
            // eat using be_decode_rec
            _ => list.push(be_decode_rec(buffer, idx)),
        }
    }

    // eat 'e'
    *idx += 1;

    BeNode::LIST(list)
}

pub fn be_decode_dict(_buffer: &Vec<char>, idx: &mut usize) -> BeNode
{
    // eat 'd'
    *idx += 1;

    // TODO eat pairs using be_decode_rec until 'e'
    BeNode::DICT(HashMap::new())
}

// looking obscure but it is simple
// It's always [ ... ].join(" ... ")
// It is very open to refactors, for now this is enough
pub fn be_encode(node: &BeNode) -> String
{
    match node {
        BeNode::NUM(nb) => ["i", &nb.to_string(), "e"].join(""),
        BeNode::STR(string) => [string.len().to_string(), string.clone()].join(":"),
        BeNode::LIST(list) => ["l",
                                &list.iter()
                                .map(|elt| be_encode(elt))
                                .collect::<Vec<String>>()
                                .join(""),
                                "e"
                                ].join(""),
        BeNode::DICT(dict) => ["d", 
                               &dict.iter()
                               .map(|(key, val)| 
                      be_encode(&BeNode::STR(key.clone())) + &be_encode(val)
                                )
                               .collect::<Vec<String>>()
                               .join(""),
                               "e"]
                               .join(""), // TODO test dicts
    }
}
