use std::io::Read;
use std::fs::File;

pub fn file_to_string(wordlist_path: &str) -> String {
    let mut file = File::open(wordlist_path)
        .expect("Could not open wordlist file");
    
    let mut buf = vec![];
    file.read_to_end(&mut buf)
        .expect("Could not read wordlist file");
    
    unsafe { String::from_utf8_unchecked(buf) }
}