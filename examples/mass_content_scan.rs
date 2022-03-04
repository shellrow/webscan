extern crate webscan;
use webscan::{UriScanner, ScanStatus, RequestMethod};
use tokio;
use std::fs::{read_to_string, read};
use std::time::Duration;
use futures::executor;

#[tokio::main]
async fn main(){
    match read_to_string("targets.txt") {
        Ok(tgt) => {
            let urls: Vec<&str> = tgt.trim().split("\n").collect();
            urls.iter().for_each(|u| {
                let mut uri_scanner = match UriScanner::new(){
                    Ok(scanner) => (scanner),
                    Err(e) => panic!("Error creating scanner: {}", e),
                };
                uri_scanner.set_base_uri(u.to_string());
                match read("content.txt") {
                    Ok(ct) => {
                        let sep = b'\n';
                        ct.split(|b| b == &sep )
                            .for_each(|c| uri_scanner.add_content(c.to_vec()));
                    },
                    Err(e) => {panic!("Could not open or find content.txt file: {}", e);}
                }
                uri_scanner.set_request_method(RequestMethod::Get);
                uri_scanner.set_timeout(Duration::from_millis(20000));
                uri_scanner.set_accept_invalid_certs(true);
                executor::block_on(uri_scanner.run_scan());
                let result = uri_scanner.get_result();
                match result.scan_status {
                    ScanStatus::Done => {
                        for (uri, status) in result.responses {
                            if status.starts_with("2") { println!("{}", uri) }
                        }
                    },
                    ScanStatus::Timeout => {println!("Timed out")},
                    _ => {println!("Error")},
                };
            });
        },
        Err(e) => {panic!("Could not open or find targets.txt file: {}", e);}
    };
}
