extern crate webscan;
use webscan::{UriScanner, ScanStatus, RequestMethod};
use tokio;
use std::fs::read_to_string;
use std::time::Duration;

#[tokio::main]
async fn main(){
    let mut uri_scanner = match UriScanner::new(){
        Ok(scanner) => (scanner),
        Err(e) => panic!("Error creating scanner: {}", e),
    };
    let base_uri = String::from("http://192.168.1.8/xvwa/");
    uri_scanner.set_base_uri(base_uri);
    let data = read_to_string("common.txt");
    let text = match data {
        Ok(content) => content,
        Err(e) => {panic!("Could not open or find file: {}", e);}
    };
    let word_list: Vec<&str> = text.trim().split("\n").collect();
    for word in word_list {
        uri_scanner.add_word(word.to_string());
    }
    uri_scanner.set_request_method(RequestMethod::Head);
    uri_scanner.set_timeout(Duration::from_millis(20000));
    uri_scanner.set_accept_invalid_certs(false);
    uri_scanner.run_scan().await;
    let result = uri_scanner.get_result();
    print!("Status: ");
    match result.scan_status {
        ScanStatus::Done => {println!("Normal end")},
        ScanStatus::Timeout => {println!("Timed out")},
        _ => {println!("Error")},
    }
    println!("URI Scan Result:");
    for (uri, status) in result.responses {
        println!("{} {}", uri, status);
    }
    println!("Scan Time: {:?}", result.scan_time);
}
