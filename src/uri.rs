use reqwest::Client;
use futures::{stream, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub enum RequestMethod {
    Get,
    Post,
}

pub async fn scan_uri(base_uri: &String, word_list: &Vec<String>, req_method: &RequestMethod) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let scan_results: Arc<Mutex<HashMap<String, String>>> = Arc::new(Mutex::new(HashMap::new()));
    let client = Client::new();
    let mut target_uris: Vec<String> = Vec::new();
    for w in word_list{
        target_uris.push(format!("{}{}", base_uri, w));
    }
    let results = stream::iter(target_uris).map(|uri| {
        let client = &client;
        async move {
            match req_method {
                RequestMethod::Get => {
                    let resp = client.get(&uri).send().await.unwrap();
                    let stat = resp.status();
                    (uri, stat.to_string())
                },
                RequestMethod::Post => {
                    let resp = client.post(&uri).send().await.unwrap();
                    let stat = resp.status();
                    (uri, stat.to_string())
                },
            }
            /*
            let resp = client.get(&uri).send().await.unwrap();
            let stat = resp.status();
            (uri, stat.to_string())
            */
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
