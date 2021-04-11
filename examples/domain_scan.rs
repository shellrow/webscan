extern crate webscan;
use webscan::DomainScanner;
use webscan::ScanStatus;
use tokio;
use std::fs::read_to_string;
use std::time::Duration;

#[tokio::main]
async fn main(){
    let mut domain_scanner = match DomainScanner::new(){
        Ok(scanner) => (scanner),
        Err(e) => panic!("Error creating scanner: {}", e),
    };
    let base_domain = String::from("example.com");
    domain_scanner.set_base_domain(base_domain);
    let data = read_to_string("namelist.txt");
    let text = match data {
        Ok(content) => content,
        Err(e) => {panic!("Could not open or find file: {}", e);}
    };
    let word_list: Vec<&str> = text.trim().split("\n").collect();
    for d in word_list{
        domain_scanner.add_word(d.to_string());
    }
    domain_scanner.set_timeout(Duration::from_millis(10000));
    domain_scanner.run_scan().await;
    let result = domain_scanner.get_result();
    print!("Status: ");
    match result.scan_status {
        ScanStatus::Done => {println!("Normal end")},
        ScanStatus::Timeout => {println!("Timed out")},
        _ => {println!("Error")},
    }
    println!("Domain Scan Result:");
    for (domain, ips) in result.domain_map {
        println!("{}", domain);
        for ip in ips{
            println!("    {}", ip);
        }
    }
    println!("Scan Time: {:?}", result.scan_time);
}
