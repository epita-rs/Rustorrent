pub mod sha;
use crate::encoding::*;
use crate::utils::sha::*;
use std::fs;
use std::io;
use std::path::Path;
use std::fs::DirEntry;

// A char is four bytes so maybe lower this bound to 64_000
pub const BLOCK_SIZE:usize = 256_000;

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


pub fn hash_folder(dir: &Path, hash: &mut Sha1) -> String {
            fs::read_dir(dir).expect("READ_DIR FAILED")
                .fold(String::new(), |acc, dir_entry| 
                                     hash_decision(dir_entry, hash, acc)
                )
}

fn hash_decision(dir_entry: Result<DirEntry, std::io::Error>,
                 hash: &mut Sha1,
                 acc:String) -> String
{
    let entry = dir_entry.unwrap();
    let path = entry.path();
    if path.is_dir() {
        acc + &hash_folder(&path, hash)
    }
    else {
        let content = fs::read_to_string(path)
            .expect("FAILED TO READ FILE");
        let f_part = content.chars()
            .take(BLOCK_SIZE - hash.size)
            .collect::<String>();
        let sec_part = content.chars()
            .skip(BLOCK_SIZE - hash.size)
            .collect::<String>();

        let mut res = acc;

        hash.update(&f_part.as_bytes());

        if hash.size == BLOCK_SIZE {
            hash.digest();
            res += &hash.digest_string();
            hash.clear();
        }

        sec_part.as_bytes().chunks(BLOCK_SIZE)
            .for_each(|slice| {
                    hash.update(&slice);
                    if hash.size == BLOCK_SIZE {
                    hash.digest();
                    res += &hash.digest_string();
                    hash.clear();
                    }
                    });
        return res;
    }
}
