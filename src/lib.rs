mod http;
mod uri;
mod domain;
mod status;

use std::time::{Duration, Instant};
use std::collections::HashMap;
use tokio::time::{timeout};

pub use status::ScanStatus;
pub use http::{RequestMethod, ResponseStatus};

/// Structure for uri scan  
/// 
/// Should be constructed using UriScanner::new
#[derive(Clone)]
pub struct UriScanner {
    /// Base URI of scan target.  
    base_uri: String,
    /// Word-list of files or directories
    word_list: Vec<String>,
    /// Request Method
    request_method: RequestMethod,
    /// Timeout setting of uri scan.  
    timeout: Duration,
    /// Accept invalid certs
    accept_invalid_certs: bool,
    /// Result of uri scan.  
    scan_result: UriScanResult,
    /// List of content buffers to find in returned pages
    content_list: Vec<Vec<u8>>,
}

/// Structure for domain scan  
/// 
/// Should be constructed using DomainScanner::new
#[derive(Clone)]
pub struct DomainScanner {
    /// Base Domain Name of scan target.  
    base_domain: String,
    /// Word-list of name
    word_list: Vec<String>,
    /// Timeout setting of domain scan.  
    timeout: Duration,
    /// Result of domain scan.  
    scan_result: DomainScanResult,
}

/// Result of UriScanner::run_scan  
#[derive(Clone)]
pub struct UriScanResult {
    /// HashMap of responses. 
    /// 
    /// (URI, Status)
    pub responses: HashMap<String, ResponseStatus>,
    /// Time from start to end of scan.  
    pub scan_time: Duration,
    /// Scan job status
    pub scan_status: ScanStatus,
}

/// Result of DomainScanner::run_scan  
#[derive(Clone)]
pub struct DomainScanResult {
    /// HashMap of domain-map. 
    /// 
    /// (Domain, IP Address)
    pub domain_map: HashMap<String, Vec<String>>,
    /// Time from start to end of scan.  
    pub scan_time: Duration,
    /// Scan job status
    pub scan_status: ScanStatus,
}

impl UriScanner{
    /// Construct new UriScanner  
    pub fn new() -> Result<UriScanner, String> {
        let ini_scan_result = UriScanResult{
            responses: HashMap::new(),
            scan_time: Duration::from_millis(0),
            scan_status: ScanStatus::Ready,
        };
        let uri_scanner = UriScanner{
            base_uri: String::new(),
            word_list: vec![],
            request_method: RequestMethod::Head,
            timeout: Duration::from_millis(30000),
            accept_invalid_certs: false,
            scan_result: ini_scan_result,
            content_list: vec![],
        };
        Ok(uri_scanner)
    }
    /// Set base URI of scan target.  
    pub fn set_base_uri(&mut self, base_uri: String) {
        self.base_uri = base_uri;
    }
    /// Add word(file name or dir name) to word-list
    pub fn add_word(&mut self, word: String) {
        if word.len() != 0 { self.word_list.push(word) }
    }
    /// Set request method
    pub fn set_request_method(&mut self, method: RequestMethod) {
        self.request_method = method;
    }
    /// Set scan timeout  
    pub fn set_timeout(&mut self, timeout: Duration){
        self.timeout = timeout;
    }
    /// Set option to accept invalid certificates
    pub fn set_accept_invalid_certs(&mut self, accept: bool) {
        self.accept_invalid_certs = accept;
    }
    /// Add content vector to search for in response bytes
    pub fn add_content(&mut self, content: Vec<u8>) {
        if content.len() != 0 { self.content_list.push(content) }
    }
    /// Run scan with current settings. 
    /// 
    /// Results are stored in UriScanner::scan_result
    pub async fn run_scan(&mut self){
        let start_time = Instant::now();
        let res = timeout(self.timeout, uri::scan_uri(&self.base_uri, &self.word_list, &self.request_method, self.accept_invalid_certs, &self.content_list)).await;
        match res {
            Ok(responses) => {
                self.scan_result.responses = responses;
                self.scan_result.scan_status = ScanStatus::Done;
            },
            Err(_) => {
                self.scan_result.scan_status = ScanStatus::Timeout;
            },
        }
        //self.scan_result.responses = uri::scan_uri(&self.base_uri, &self.word_list).await;
        self.scan_result.scan_time = Instant::now().duration_since(start_time);
    }
    /// Return scan result.
    pub fn get_result(&mut self) -> UriScanResult{
        return self.scan_result.clone();
    }
}

impl DomainScanner {
    /// Construct new UriScanner  
    pub fn new() -> Result<DomainScanner, String> {
        let ini_scan_result = DomainScanResult {
            domain_map: HashMap::new(), 
            scan_time: Duration::from_millis(0),
            scan_status: ScanStatus::Ready,
        };
        let domain_scanner = DomainScanner {
            base_domain: String::new(),
            word_list: vec![],
            timeout: Duration::from_millis(30000),
            scan_result: ini_scan_result,
        };
        Ok(domain_scanner)
    }
    /// Set base Domain of scan target.  
    pub fn set_base_domain(&mut self, base_domain: String) {
        self.base_domain = base_domain;
    }
    /// Add word to word-list
    pub fn add_word(&mut self, word: String) {
        self.word_list.push(word);
    }
    /// Set scan timeout  
    pub fn set_timeout(&mut self, timeout: Duration){
        self.timeout = timeout;
    }
    /// Run scan with current settings. 
    /// 
    /// Results are stored in DomainScanner::scan_result
    pub async fn run_scan(&mut self){
        let start_time = Instant::now();
        let res = timeout(self.timeout, domain::scan_domain(&self.base_domain, &self.word_list)).await;
        match res {
            Ok(domain_map) => {
                self.scan_result.domain_map = domain_map;
                self.scan_result.scan_status = ScanStatus::Done;
            },
            Err(_) => {
                self.scan_result.scan_status = ScanStatus::Timeout;
            },
        }
        //self.scan_result.domain_map = domain::scan_domain(&self.base_domain, &self.word_list).await;
        self.scan_result.scan_time = Instant::now().duration_since(start_time);
    }
    /// Return scan result.
    pub fn get_result(&mut self) -> DomainScanResult{
        return self.scan_result.clone();
    }
}
