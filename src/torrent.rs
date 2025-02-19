use crate::encoding::*;
use crate::utils::*;
use crate::BeSTR;
use std::path::Path;
use std::time::SystemTime;

pub struct Torrent {
    // MANDATORY BEP 003
    // https://www.bittorrent.org/beps/bep_0003.html
    announce:String, // domain name
    creationDate:usize, // seconds
    author:String,
    filename:String,
    pieces_len:usize, // number of pieces
    pieces:String, // concatenation of hashs
}

impl Torrent {
    fn new(path:String) -> Torrent
    {
        let path = Path::new(&path);

        if !path.exists() {
            panic!("The given path is not valid.");
        }

        let mut dict = vec![(String::from("name"), BeSTR!(stringify!(path)))];

        if path.is_dir() {
            let mut files = BeNode::LIST(vec![]);
            build_torrent(path, &mut vec![], &mut files);
            // TODO total length
            // TODO progressively hash the folder/file content
        }
        else
        {
            let length = BeNode::NUM(path.metadata()
                                         .expect("METADATA FAILED")
                                         .len() as i64);
            dict.push((String::from("length"), length));
        }

        let info = BeNode::DICT(dict);

        let announce = String::from("http://localhost:8080"); 
        let creationDate = SystemTime::now()
                                      .duration_since(SystemTime::UNIX_EPOCH)
                                      .expect("UNIX TIME WENT BOOM")
                                      .as_secs() as usize;
        let author = String::from("Gustavo");
        let filename = String::from(stringify!(path.file_name()));

        let _node = BeNode::DICT(vec![(String::from("announce"), 
                                    BeSTR!(announce.clone()))
                                    ,(String::from("info"), info) 
                                    ]);

        let pieces = String::new();
        let pieces_len = 0;

        // TODO MAYBE sort in alphabetic order
        Torrent {
             announce, // domain name
             creationDate, // seconds
             author,
             filename,
             pieces_len, // number of pieces
             pieces, // concatenation of hashs
        }
    }
}
