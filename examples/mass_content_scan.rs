extern crate webscan;
use webscan::{UriScanner, ScanStatus, RequestMethod};
use tokio;
use std::fs::{read_to_string, read};
use std::time::Duration;

#[tokio::main]
async fn main(){
    // Set up base scanner template struct to clone
    let mut base_scanner = match UriScanner::new(){
        Ok(scanner) => scanner,
        Err(e) => panic!("Error creating scanner: {}", e),
    };
    base_scanner.set_request_method(RequestMethod::Get);
    base_scanner.set_timeout(Duration::from_millis(20000));
    base_scanner.set_accept_invalid_certs(true);
    // Get byte vectors of content
    match read("content.txt") {
        Ok(ct) => {
            let sep = b'\n';
            ct.split(|b| b == &sep )
                .for_each(|c| base_scanner.add_content(c.to_vec()));
        },
        Err(e) => {panic!("Could not open or find content.txt file: {}", e);}
    };
    // Get strings of target URLs    
    match read_to_string("targets.txt") {
        Ok(tgt) => {
            let urls: Vec<&str> = tgt.trim().split("\n").collect();
            // Build set of scanners for all targets
            let mut scanners: Vec<UriScanner> = urls.iter().map(|u| {
                let mut scanner = base_scanner.clone();
                scanner.set_base_uri(u.to_string());
                scanner
            }).collect();
            // Run scanners in parallel
            while let Some(mut scanner) = scanners.pop() {
                tokio::spawn(async move {
                    scanner.run_scan().await;
                    let result = scanner.get_result();
                    match result.scan_status {
                        ScanStatus::Done => {
                            for (uri, status) in result.responses {
                                // Content matches are "given" a 200 response string
                                if status.starts_with("2") { println!("{}", uri) }
                            }
                        },
                        ScanStatus::Timeout => {println!("Timed out")},
                        _ => {println!("Error")},
                    };
                });
            };
        },
        Err(e) => {panic!("Could not open or find targets.txt file: {}", e);}
    };
}
