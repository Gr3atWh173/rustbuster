use std::{fs::File, io::Write, path::Path, str};

use super::result_processor::SingleVhostScanResult;
use crate::common_utils::file_to_string;

pub fn build_vhosts(wordlist_path: &str, url: &str) -> Vec<hyper::Uri> {
    debug!("building urls");
    let mut urls: Vec<hyper::Uri> = Vec::new();
    let wordlist = file_to_string(wordlist_path);
    let urls_iter = wordlist
        .lines()
        .filter(|word| !word.starts_with('#') && !word.starts_with(' '))
        .map(|word| format!("{}.{}", word, url));

    for url in urls_iter {
        match url.parse::<hyper::Uri>() {
            Ok(v) => {
                urls.push(v);
            }
            Err(e) => {
                trace!("URI: {}", e);
            }
        }
    }

    urls
}

pub fn save_vhost_results(path: &str, results: &Vec<SingleVhostScanResult>) {
    let json_string = serde_json::to_string(&results).unwrap();

    let mut file = match File::create(Path::new(path)) {
        Ok(f) => f,
        Err(e) => {
            error!("Error while creating file: {}\n{}", path, e);
            return;
        }
    };

    match file.write_all(json_string.as_bytes()) {
        Ok(_) => debug!("Results saved to: {}", path),
        Err(e) => error!("Error while writing results to file: {}\n{}", path, e),
    };
}
