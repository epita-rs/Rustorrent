#[cfg(test)]
mod unit {
    use super::*;
    use crate::encoding::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::collections::HashMap;

    macro_rules! dict {
        ($pairs:expr) => {
            {
                let mut dict:HashMap<String, BeNode> = HashMap::new();

                for (key, val) in $pairs {
                    dict.insert(String::from(key), BeNode::STR(String::from(val)));
                }
                dict
            }
        }
    }

    macro_rules! encoding_tests {
        ($($name:ident: $args:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (data, expected) = $args;
                    let result = be_encode(&data);
                    assert_eq!(result, expected);
                }
            )*
        }
    }

    // TODO for later use
    macro_rules! test_from_files {
        ($($name:ident: $args:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (path, expected) = $args;

                    let mut file = File::open(path).expect("failed to open");
                    let mut content = String::new();
                    file.read_to_string(&mut content).expect("failed to read");

                    let result = "dummy"; 
                    assert_eq!(result, expected);
                }
            )*
        }
    }
    // =====================================================================

        encoding_tests! {
        number_easy: (
        BeNode::NUM(1234),
        "i1234e"
        ),
        number_medium: (
        BeNode::NUM(123456789),
        "i123456789e"
        ),
        number_neg: (
        BeNode::NUM(-1234),
        "i-1234e"
        ),
        number_zero: (
        BeNode::NUM(0),
        "i0e"
        ),
        number_neg_zero: (
        BeNode::NUM(-0),
        "i0e"
        ),
        string_start: (
        BeNode::STR(String::from("hola")),
        "4:hola"
        ),
        string_long: (
        BeNode::STR(String::from("aaabbbcccdddeeefffggghhhjjj")),
        "27:aaabbbcccdddeeefffggghhhjjj"
        ),
        string_number: (
        BeNode::STR(String::from("123456")),
        "6:123456"
        ),
        string_single: (
        BeNode::STR(String::from("e")),
        "1:e"
        ),
        string_more: (
        BeNode::STR(String::from("hola---hola")),
        "11:hola---hola"
        ),
        list_basic: (
        BeNode::LIST(vec![BeNode::NUM(123), BeNode::NUM(456), BeNode::NUM(0)]),
        "li123ei456ei0ee"
        ),
        list_mixed: (
        BeNode::LIST(vec![BeNode::STR(String::from("hola")),
                          BeNode::NUM(123)]),
        "l4:holai123ee"
        ),
        list_strings: (
        BeNode::LIST(vec![BeNode::STR(String::from("hola")),
                          BeNode::STR(String::from("hello")),
                          BeNode::STR(String::from("ciao"))]),
        "l4:hola5:hello4:ciaoe"
        ),
        dict_simple: (
           BeNode::DICT(dict!(vec![("hola", "Pedro")])),
           "d4:hola5:Pedroe"
        ),
        dict_three: (
           BeNode::DICT(dict!(vec![("hola", "Pedro"),
                                   ("hello", "William"), 
                                   ("|", "|")])),
           "d4:hola5:Pedro5:hello7:William1:|1:|e"
        ),
    }
}
