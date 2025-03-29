pub mod sha;
use crate::encoding::*;
use crate::utils::sha::*;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::collections::BTreeMap;

// A char is four bytes so maybe lower this bound to 64_000
pub const BLOCK_SIZE:usize = 256_000;

#[macro_export]
macro_rules! BeSTR {
    ($str:expr) => {
        BeNode::STR(String::from($str))
    } 
}

pub fn split_path(path: &Path) -> Vec<BeNode> {
    path.components().map(|comp| 
        BeNode::STR(String::from(comp.as_os_str().to_str().unwrap()))
    ).collect::<Vec<BeNode>>()
}

pub fn build_torrent(dir: &Path,
                    split_path:&mut Vec<BeNode>,
                    node: &mut BeNode) -> io::Result<()>{
        if dir.is_dir()  {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                split_path.push(BeSTR!(entry.file_name().into_string().unwrap()));

                let path = entry.path();

                if path.is_dir() {
                    let _ = build_torrent(&path, split_path, node);
                }
                else
                {
                    match node {
                        BeNode::LIST(list) => {
                            let size = entry.metadata()?.len() as i64;

                            let dict = BTreeMap::from([
                                (String::from("length"), BeNode::NUM(size)),
                                (String::from("path"), BeNode::LIST(split_path.clone()))
                            ]); 

                            list.push(BeNode::DICT(dict)); 
                        },
                        _ => panic!("Error while building .torrent"),
                    }
                }
                split_path.pop();
            }
        }
        Ok(())
}

// builds the relative path of a file from its dictionary representation
pub fn filepath_concat(relpath:&PathBuf, list: &Vec<BeNode>) -> PathBuf {
    let mut path = PathBuf::new();
    path.push(relpath);
    list.iter().for_each(|str_node|
        match str_node { 
            BeNode::STR(folder_name) => path.push(folder_name), 
            _ => panic!("expecting BeNode::STR")
        }
    );
    path
}

// From a BitTorrent node, produces the unique hash of the subfiles, in the order they appear 
pub fn produce_pieces_hash(relpath:PathBuf, node_list: &BeNode) -> String {
    let mut hash = Sha1::new();

    match node_list {
        BeNode::LIST(list) => {
            let mut pieces = list.iter().fold(String::new(), |acc, node_dict| match node_dict {
                BeNode::DICT(dict) => match dict.get("path") {
                    Some(BeNode::LIST(files)) => acc + &hash_decision(filepath_concat(&relpath, files), &mut hash),
                    _ => panic!("missing the path of the file")
                },
                _ => panic!("expecting BeNode::DICT")
            });
            if hash.size != 0 {
                pieces += &hash.digest_raw();
            }
            pieces
        },
        _ => panic!("expecting BeNode::LIST")
    }
}

fn digest_if_full(hash: &mut Sha1, res: &mut String)
{
    // empties hash if full
    if hash.size == BLOCK_SIZE {
            hash.digest();
            *res += &hash.digest_raw();
            hash.clear();
    }
}

fn hash_decision(path:PathBuf,
                 hash: &mut Sha1) -> String
{
    println!("{}", path.display());
    let content = match fs::read_to_string(path) {
        Ok(str) => str,
        Err(_) => panic!("FAILED TO READ PATH: ")
    };

    // content splitting
    let missing_hdata = BLOCK_SIZE - hash.size;
    let f_part = content.chars()
        .take(missing_hdata)
        .collect::<String>();
    let sec_part = content.chars()
        .skip(missing_hdata)
        .collect::<String>();

    let mut res = String::new();

    hash.update(&f_part.as_bytes());
    digest_if_full(hash, &mut res);

    // here the hash is guaranted to be empty, so we can put one block at a time
    sec_part.as_bytes().chunks(BLOCK_SIZE)
        .for_each(|slice| {
                hash.update(&slice);
                digest_if_full(hash, &mut res);
            });

    res
}

/*
#[cfg(test)]
pub mod unit_utils {

}
 */