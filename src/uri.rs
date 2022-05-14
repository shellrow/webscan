use reqwest::Client;
use futures::{stream, StreamExt};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use crate::http::{RequestMethod, ResponseStatus};

fn content_contains(content: &[u8], elem: &[u8]) -> bool {
    if elem.len() == 0 { return false }
    match content.windows(elem.len())
        .position(|w| w == elem) {
            Some(_) => true,
            None => false
        }
}

pub async fn scan_uri(base_uri: &String, word_list: &Vec<String>, req_method: &RequestMethod, accept_invalid_certs: bool, content_list: &Vec<Vec<u8>>, ptx: &Arc<Mutex<Sender<String>>>) -> HashMap<String, ResponseStatus> {
    let mut result: HashMap<String, ResponseStatus> = HashMap::new();
    let scan_results: Arc<Mutex<HashMap<String, ResponseStatus>>> = Arc::new(Mutex::new(HashMap::new()));
    let client = if accept_invalid_certs {
        Client::builder().danger_accept_invalid_certs(true).build().unwrap()
    }else {
        Client::new()
    };
    let mut target_uris: Vec<String> = Vec::new();
    for w in word_list{
        target_uris.push(format!("{}{}", base_uri, w));
    }
    if target_uris.len() == 0 {
        target_uris.push(base_uri.to_owned());
    }
    let results = stream::iter(target_uris).map(|uri| {
        let client = &client;
        async move {
            let req = match req_method {
                RequestMethod::Head => client.head(&uri).send(),
                RequestMethod::Post => client.post(&uri).send(),
                RequestMethod::Get => client.get(&uri).send(),
            };
            let r = match req.await {
                Ok(resp) => {
                    let stat = if content_list.len() == 0 {
                        ResponseStatus::from_code(resp.status().as_u16()) 
                    } else {
                        let content = &resp.bytes().await.unwrap();
                        match content_list.iter().find(|&elem| content_contains(&content,&elem)) {
                            Some(_) => ResponseStatus::from_code(200), // do not lose if matched but 404
                            None => ResponseStatus::from_code(404)
                        }
                    };
                    (uri.clone(), stat)
                },
                Err(_) => {
                    (uri.clone(), ResponseStatus::from_code(404))
                }
            };
            match ptx.lock() {
                Ok(lr) => {
                    match lr.send(uri) {
                        Ok(_) => {},
                        Err(_) => {},
                    }
                },
                Err(_) => {},
            }
            r
        }
    }).buffer_unordered(100);

    results.for_each(|r| async {
        match r {
            (uri, status) => {
                if status != ResponseStatus::NotFound {
                    scan_results.lock().unwrap().insert(uri, status);
                }
            },
        }
    }).await;

    for (uri, status) in scan_results.lock().unwrap().iter() {
        result.insert(uri.to_owned(), status.to_owned());
    }
    return result;
}
