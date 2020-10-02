mod models;
mod pool;
use crate::models::endpoint::{DataEndpoint, Endpoint, HomeEndpoint};
use crate::models::node::Node;
use crate::models::token::Token;
use crate::pool::threadpool::ThreadPool;
use reqwest::header;
use reqwest::{self, Error};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

fn make_request(
    client: reqwest::blocking::Client,
    url: String,
    acc_tok: String,
) -> Result<String, Error> {
    let request = client.get(url.as_str()).header(
        "X-Access-Token",
        header::HeaderValue::from_str(acc_tok.as_str()).unwrap(),
    );
    let response = request.send()?.text()?;
    Ok(response)
}

fn main() -> Result<(), Error> {
    let localhost = "http://localhost:5000";
    let body = reqwest::blocking::get(localhost)?.text()?;
    let endpoints: HashMap<String, Endpoint> = serde_json::from_str(&body.as_str()).unwrap();
    let mut access_token: Token = Token {
        access_token: String::from(""),
    };
    if let Some(url) = endpoints.get(&String::from("register")) {
        let register_url = url;
        let body = reqwest::blocking::get(format!("{}{}", localhost, register_url.link).as_str())?
            .text()?;
        access_token = serde_json::from_str(&body.as_str()).unwrap();
    }
    if let Some(url) = endpoints.get(&String::from("home")) {
        let home_url = url;
        let client = reqwest::blocking::Client::new();
        let request = client
            .get(format!("{}{}", localhost, home_url.link).as_str())
            .header(
                "X-Access-Token",
                header::HeaderValue::from_str(access_token.access_token.as_str()).unwrap(),
            );
        let response = request.send()?.text()?;
        let endpoints_2: HomeEndpoint = serde_json::from_str(&response.as_str()).unwrap();
        let pool = ThreadPool::new(6);
        let all_data_records: Arc<Mutex<VecDeque<DataEndpoint>>> =
            Arc::new(Mutex::new(VecDeque::default()));
        for point in endpoints_2.link {
            let all_routes: Arc<Mutex<VecDeque<DataEndpoint>>> =
                Arc::new(Mutex::new(VecDeque::default()));
            let routes_clone = Arc::clone(&all_routes);
            let data_records_clone = Arc::clone(&all_data_records);
            let acc_tok = access_token.access_token.clone();
            pool.execute(move || {
                let inner_routes_clone = Arc::clone(&routes_clone);
                let client = reqwest::blocking::Client::new();

                let url = format!("{}{}", localhost, point.1.as_str());
                let response = make_request(client, url, acc_tok.clone());
                let response = match response {
                    Ok(resp) => resp,
                    Err(err) => panic!("{}", err),
                };

                let endpoint: DataEndpoint = serde_json::from_str(&response.as_str()).unwrap();
                let mut sub_routes_vec = inner_routes_clone.lock().unwrap();
                sub_routes_vec.push_back(endpoint);
                drop(sub_routes_vec);
                while inner_routes_clone.lock().unwrap().len() != 0 {
                    let mut n = inner_routes_clone.lock().unwrap().len();
                    while n > 0 {
                        let child_node = inner_routes_clone.lock().unwrap().pop_front();
                        let child_node = match child_node {
                            Some(data) => data,
                            None => panic!("wtf"),
                        };
                        data_records_clone
                            .lock()
                            .unwrap()
                            .push_back(child_node.clone());
                        let pool_2 = ThreadPool::new(5);
                        for route in child_node.link {
                            let inner_routes_clone = Arc::clone(&routes_clone);
                            let acc_tok = acc_tok.clone();
                            pool_2.execute(move || {
                                let client = reqwest::blocking::Client::new();
                                let url = format!("{}{}", localhost, route.1.as_str());
                                let response = make_request(client, url, acc_tok.clone());
                                let response = match response {
                                    Ok(resp) => resp,
                                    Err(err) => panic!("{}", err),
                                };
                                let endpoint: DataEndpoint =
                                    serde_json::from_str(&response.as_str()).unwrap();
                                let mut routes_queue = inner_routes_clone.lock().unwrap();
                                routes_queue.push_back(endpoint);
                            });
                        }
                        drop(pool_2);
                        n -= 1;
                    }
                }
            });
        }
        drop(pool);
        println!("{:?}", all_data_records.lock().unwrap().len());
    }
    Ok(())
}
