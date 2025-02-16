use std::collections::HashMap;

#[derive(Debug)]
pub enum BeNode
{
    NUM(i64),
    STR(String),
    LIST(Vec<BeNode>),
    DICT(HashMap<String, BeNode>),
}

pub fn be_decode(_buffer: String) -> BeNode
{
    BeNode::NUM(0)
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
