extern crate webscan;
use webscan::DomainScanner;
use webscan::ScanStatus;
use tokio;
use tokio::runtime::Runtime;
use std::fs::read_to_string;
use std::time::Duration;
use std::thread;

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
    let rx = domain_scanner.get_progress_receiver();
    let rt = Runtime::new().unwrap();
    // Run scan 
    let handle = thread::spawn(move|| {
        rt.block_on(async {
            domain_scanner.scan().await
        })
    });
    // Print progress
    while let Ok(_domain) = rx.lock().unwrap().recv() {
        //println!("Check: {}", domain);
    }
    let result = handle.join().unwrap();
    print!("Status: ");
    match result.scan_status {
        ScanStatus::Done => {println!("Done")},
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
