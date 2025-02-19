use crate::encoding::*;
use std::fs;
use std::io;
use std::path::Path;

#[macro_export]
macro_rules! BeSTR {
    ($str:expr) => {
        BeNode::STR(String::from($str))
    } 
}

pub fn build_torrent(dir: &Path,
                    split_path:&mut Vec<BeNode>,
                    node: &mut BeNode) -> io::Result<()>{
        if dir.is_dir()  {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                split_path.push(BeSTR!(stringify!(entry.file_name())));

                let path = entry.path();

                if path.is_dir() {
                    let _ = build_torrent(&path, split_path, node);
                }
                else
                {
                    match node {
                        BeNode::LIST(list) => {
                            let mut dict:Vec<(String, BeNode)>  = vec![]; 

                            let size = entry.metadata()?.len() as i64;
                            dict.push((String::from("length"), BeNode::NUM(size)));

                            dict.push((String::from("path"),
                                       BeNode::LIST(split_path.clone())));

                            list.push(BeNode::DICT(dict)); 
                        },
                        _ => panic!("Error while building .torrent"),
                    }
                }
            }
        }
        Ok(())
}
