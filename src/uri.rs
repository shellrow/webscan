use reqwest::Client;
use futures::{stream, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// HTTP request method for scanning
pub enum RequestMethod {
    Get,
    Post,
    Head,
}

pub async fn scan_uri(base_uri: &String, word_list: &Vec<String>, req_method: &RequestMethod, accept_invalid_certs: bool) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let scan_results: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let client = if accept_invalid_certs {
        Client::builder().danger_accept_invalid_certs(true).build().unwrap()
    }else {
        Client::new()
    };
    let mut target_uris: Vec<String> = Vec::new();
    for w in word_list{
        target_uris.push(format!("{}{}", base_uri, w));
    }
    let results = stream::iter(target_uris).map(|uri| {
        let client = &client;
        async move {
            let resp = match req_method {
                RequestMethod::Head => client.head(&uri).send().await.unwrap(),
                RequestMethod::Post => client.post(&uri).send().await.unwrap(),
                RequestMethod::Get => client.get(&uri).send().await.unwrap(),
            };
            let stat = resp.status();
            (uri, stat.to_string())
        }
    }).buffer_unordered(100);

    results.for_each(|r| async {
        match r {
            (uri, status) => {
                if status != "404 Not Found" {
                    scan_results.lock().unwrap().insert(uri, status);
                }
            },
        }
    }).await;

    for (uri, status) in scan_results.lock().unwrap().iter() {
        result.insert(uri.to_string(), status.to_string());
    }
    return result;
}
