use futures::{stream, StreamExt};
use dns_lookup::lookup_host;
use std::net::IpAddr;
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::{Arc, Mutex};

pub async fn scan_domain(base_domain: &String, word_list: &Vec<String>, ptx: &Arc<Mutex<Sender<String>>>) -> HashMap<String, Vec<String>> {
    let mut result = HashMap::new();
    let scan_results: Arc<Mutex<HashMap<String, Vec<String>>>> = Arc::new(Mutex::new(HashMap::new()));
    let mut target_domain: Vec<String> = Vec::new();
    for w in word_list {
        target_domain.push(format!("{}.{}", w, base_domain));
    }
    let results = stream::iter(target_domain).map(|domain| {
        let mut ip_list:Vec<String> = Vec::new();
        async move {
            match lookup_host(&domain) {
                Ok(ips) =>{
                    for ip in ips{
                        match ip {
                            IpAddr::V4(ipv4) => {
                                ip_list.push(ipv4.to_string());
                            },
                            IpAddr::V6(ipv6) => {
                                ip_list.push(ipv6.to_string());
                            }
                        }
                    }
                },
                Err(_) => {

                }
            }
            match ptx.lock() {
                Ok(lr) => {
                    match lr.send(domain.clone()) {
                        Ok(_) => {},
                        Err(_) => {},
                    }
                },
                Err(_) => {},
            }
            (domain, ip_list)
        }
    }).buffer_unordered(100);

    results.for_each(|r| async {
        match r {
            (domain, ips) => {
                if ips.len() > 0 {
                    scan_results.lock().unwrap().insert(domain, ips);
                }
            },
        }
    }).await;

    for (domain, ips) in scan_results.lock().unwrap().iter() {
        result.insert(domain.to_string(), ips.clone());
    }
    return result;
}