use reqwest::header;
use reqwest::{self, Error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::{fmt, thread};

#[derive(Serialize, Deserialize, Debug)]
struct Endpoint {
    description: String,
    link: String,
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.link, self.description)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Token {
    access_token: String,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, rx: Arc<Mutex<mpsc::Receiver<Job>>>) -> Self {
        let thread = thread::spawn(move || {
            let job = rx.lock().unwrap().recv().unwrap();
            job();
        });
        Self {
            id: id,
            thread: thread::spawn(|| {
                thread;
            }),
        }
    }
}
struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> Self {
        assert!(size > 0);
        let (tx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));
        let mut workers = Vec::with_capacity(size);
        for i in 0..size {
            workers.push(Worker::new(i, Arc::clone(&rx)));
        }
        ThreadPool {
            workers,
            sender: tx,
        }
    }

    fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

fn main() -> Result<(), Error> {
    let localhost = "http://localhost:5000";
    let body = reqwest::blocking::get(localhost)?.text()?;
    let endpoints: HashMap<String, Endpoint> = serde_json::from_str(&body.as_str()).unwrap();
    println!("{:?}", endpoints);
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
        println!("{:?}", access_token.access_token);
        let request = reqwest::blocking::Client::new()
            .get(format!("{}{}", localhost, home_url.link).as_str())
            .header(
                "X-Access-Token",
                header::HeaderValue::from_str(access_token.access_token.as_str()).unwrap(),
            );
        let response = request.send()?.text()?;
        println!("{:?}", response);
        let endpoints_2: HashMap<String, Option<Endpoint>> =
            serde_json::from_str(&response.as_str()).unwrap();
        println!("{:?}", endpoints_2);
    }
    // testing thread pool
    let localhost = "http://localhost:5000";
    let client = Arc::new(Mutex::new(reqwest::blocking::Client::new()));
    client.lock().unwrap().get(localhost);
    let pool = ThreadPool::new(5);

    for i in 0..10 {
        let client_copy = Arc::clone(&client);
        pool.execute(move || {
            println!("hopa");
            let req = client_copy.lock().unwrap().get(localhost);
            println!("worker {} done", i);
            println!("{:?}", req);
        });
    }

    std::thread::sleep(std::time::Duration::from_secs(1));

    Ok(())
}
