use std::env;
#[cfg(test)]
mod unit {
    use super::*;
    use crate::encoding::*;
    use crate::torrent::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::collections::BTreeMap;

    macro_rules! dict {
        ($pairs:expr) => {
            {
                let dict = $pairs;

                dict.iter()
                    .map(|(key, val)| 
                            (String::from(*key),
                             BeNode::STR(String::from(*val)))
                        )
                    .collect::<BTreeMap<String, BeNode>>()
            }
        }
    }

    macro_rules! encoding_tests {
        ($($decode_name:ident: $encode_name:ident: $args:expr,)*) => {
            $(
                #[test]
                fn $encode_name() {
                    let (data, expected) = $args;
                    let result = be_encode(&data);
                    assert_eq!(result, expected);
                }

                #[test]
                fn $decode_name() {
                    let (expected, data) = $args;
                    let result = be_decode(String::from(data));
                    assert_eq!(result, expected);
                }
            )*
        }
    }
    macro_rules! torrent_dir {
        ($($name:ident: $args:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let (path, expected) = $args;

                    let torrent = Torrent::newNode(String::from(path));

                    assert!(env::set_current_dir(&path).is_ok());
                    println!("The current directory is {}", env::current_dir().unwrap().display());

                    match torrent {
                        BeNode::DICT(dict) => {
                            if let Some(info) = dict.get("info") {
                                let result = be_encode(&info);
                                assert_eq!(result, expected);
                            }
                            else {
                              panic!("info not found")  
                            }
                        } ,
                        _ => panic!("BENODE::DICT not found")
                    };
                    
                }
            )*
        }
    }

    // TODO for later use
    macro_rules! _test_from_files {
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

    torrent_dir! {
        torrent_classic: (
        "src/testsuite/test_material/test1",
        "d5:filesld6:lengthi22e4:pathl5:b.txteed6:lengthi16e4:pathl5:a.txteed6:lengthi33e4:pathl6:nested3:d.ceed6:lengthi44e4:pathl3:asm6:fibo.Seee4:name5:test112:piece lengthi16384e6:pieces20:j����,��serZM���Vee"
        ),
    }

    encoding_tests! {
        enc_number_easy:dec_number_easy: (
        BeNode::NUM(1234),
        "i1234e"
        ),
        enc_number_medium:dec_number_medium: (
        BeNode::NUM(123456789),
        "i123456789e"
        ),
        enc_number_neg:dec_number_neg: (
        BeNode::NUM(-1234),
        "i-1234e"
        ),
        enc_number_zero:dec_number_zero: (
        BeNode::NUM(0),
        "i0e"
        ),
        enc_number_neg_zero:dec_number_neg_zero: (
        BeNode::NUM(-0),
        "i0e"
        ),
        enc_string_start:dec_string_start: (
        BeNode::STR(String::from("hola")),
        "4:hola"
        ),
        enc_string_long:dec_string_long: (
        BeNode::STR(String::from("aaabbbcccdddeeefffggghhhjjj")),
        "27:aaabbbcccdddeeefffggghhhjjj"
        ),
        enc_string_number:dec_string_number: (
        BeNode::STR(String::from("123456")),
        "6:123456"
        ),
        enc_string_single:dec_string_single: (
        BeNode::STR(String::from("e")),
        "1:e"
        ),
        enc_string_more:dec_string_more: (
        BeNode::STR(String::from("hola---hola")),
        "11:hola---hola"
        ),
        enc_list_basic:dec_list_basic: (
        BeNode::LIST(vec![BeNode::NUM(123), BeNode::NUM(456), BeNode::NUM(0)]),
        "li123ei456ei0ee"
        ),
        enc_list_mixed:dec_list_mixed: (
        BeNode::LIST(vec![BeNode::STR(String::from("hola")),
                          BeNode::NUM(123)]),
        "l4:holai123ee"
        ),
        enc_list_strings:dec_list_strings: (
        BeNode::LIST(vec![BeNode::STR(String::from("hola")),
                          BeNode::STR(String::from("hello")),
                          BeNode::STR(String::from("ciao"))]),
        "l4:hola5:hello4:ciaoe"
        ),
        enc_dict_simple: dec_dict_simple: (
           BeNode::DICT(dict!(vec![("hola", "Pedro")])),
           "d4:hola5:Pedroe"
        ),
        enc_dict_three: dec_dict_three: (
           BeNode::DICT(dict!(vec![("hola", "Pedro"),
                                   ("hello", "William"), 
                                   ("s", "o"), 
                                   ("|", "|")])),
           "d5:hello7:William4:hola5:Pedro1:s1:o1:|1:|e"
        ),
        enc_dict_mix: dec_dict_mix: (
           BeNode::DICT(BTreeMap::from([
                            (String::from("hola"), BeNode::STR(String::from("Pedro"))),
                            (String::from("nb"), BeNode::NUM(10)),
                            (String::from("list"), BeNode::LIST(vec![BeNode::NUM(11), BeNode::NUM(12)])),
                            (String::from("dict"), BeNode::DICT(dict!(vec![("hola", "Pedro")]))),
                            ])),   
           "d4:dictd4:hola5:Pedroe4:hola5:Pedro4:listli11ei12ee2:nbi10ee"
        ),
    }
}
