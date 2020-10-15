use crate::models::datarecord::Record;
use crate::models::endpoint::{DataEndpoint, Endpoint, HomeEndpoint};
use crate::models::token::Token;
use csv;
use regex;
use reqwest::header;
use reqwest::{self, Error};
use serde_xml_rs;
use serde_yaml;
use std::collections::{HashMap, VecDeque};
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::MutexGuard;
#[macro_use(lazy_static)]
use lazy_static;

pub fn make_request(
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

pub fn handle_query(query: std::borrow::Cow<str>, stream: &mut TcpStream, records: &Vec<Record>) {
    let mut query = query.split_whitespace();
    match query.next() {
        Some(_) => match query.next() {
            Some(column) => match column {
                "id" => {
                    for record in records {
                        stream
                            .write(format!("id: {}\n", record.id).as_bytes())
                            .unwrap();
                    }
                }
                "first_name" => {
                    for record in records {
                        if record.first_name != String::from("") {
                            stream
                                .write(format!("First Name: {}\n", record.first_name).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "last_name" => {
                    for record in records {
                        if record.last_name != String::from("") {
                            stream
                                .write(format!("Last Name: {}\n", record.last_name).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "full_name" => {
                    for record in records {
                        if record.full_name != String::from("") {
                            stream
                                .write(format!("Full Name: {}\n", record.full_name).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "email" => {
                    for record in records {
                        if record.email != String::from("") {
                            stream
                                .write(format!("Email: {}\n", record.email).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "employee_id" => {
                    for record in records {
                        if record.employee_id != String::from("") {
                            stream
                                .write(format!("Employee Id: {}\n", record.employee_id).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "bitcoin_address" => {
                    for record in records {
                        if record.bitcoin_address != String::from("") {
                            stream
                                .write(
                                    format!("Bitcoin Address: {}\n", record.bitcoin_address)
                                        .as_bytes(),
                                )
                                .unwrap();
                        }
                    }
                }
                "gender" => {
                    for record in records {
                        if record.gender != String::from("") {
                            stream
                                .write(format!("Gender: {}\n", record.gender).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "ip_address" => {
                    for record in records {
                        if record.ip_address != String::from("") {
                            stream
                                .write(format!("Ip Address: {}\n", record.ip_address).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "organization" => {
                    for record in records {
                        if record.organization != String::from("") {
                            stream
                                .write(
                                    format!("Organization: {}\n", record.organization).as_bytes(),
                                )
                                .unwrap();
                        }
                    }
                }
                _ => {
                    stream.write("invalid column\n".as_bytes()).unwrap();
                }
            },
            None => {
                stream
                    .write("Please provide a column\n".as_bytes())
                    .unwrap();
            }
        },
        None => {
            stream.write("smth bad happened\n".as_bytes()).unwrap();
        }
    }
}

pub fn handle_regex_query(
    query: std::borrow::Cow<str>,
    stream: &mut TcpStream,
    records: &Vec<Record>,
) {
    let mut query = query.split_whitespace();
    let command = query.next();
    let column = query.next();
    let pattern = query.next();
    let re: regex::Regex;
    match pattern {
        Some(pattern) => re = regex::Regex::new(pattern).expect("Invalid regex"),
        None => {
            stream
                .write("Please specify glob pattern".as_bytes())
                .unwrap();
            return;
        }
    }
    match command {
        Some(_) => match column {
            Some(column) => match column {
                "id" => {
                    for record in records {
                        if re.is_match(record.id.to_string().as_str()) {
                            stream
                                .write(format!("id: {}\n", record.id).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "first_name" => {
                    for record in records {
                        if record.first_name != String::from("")
                            && re.is_match(record.first_name.as_str())
                        {
                            stream
                                .write(format!("First Name: {}\n", record.first_name).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "last_name" => {
                    for record in records {
                        if record.last_name != String::from("")
                            && re.is_match(record.last_name.as_str())
                        {
                            stream
                                .write(format!("Last Name: {}\n", record.last_name).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "full_name" => {
                    for record in records {
                        if record.full_name != String::from("")
                            && re.is_match(record.full_name.as_str())
                        {
                            stream
                                .write(format!("Full Name: {}\n", record.full_name).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "email" => {
                    for record in records {
                        if record.email != String::from("") && re.is_match(record.email.as_str()) {
                            stream
                                .write(format!("Email: {}\n", record.email).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "employee_id" => {
                    for record in records {
                        if record.employee_id != String::from("")
                            && re.is_match(record.employee_id.as_str())
                        {
                            stream
                                .write(format!("Employee Id: {}\n", record.employee_id).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "bitcoin_address" => {
                    for record in records {
                        if record.bitcoin_address != String::from("")
                            && re.is_match(record.bitcoin_address.as_str())
                        {
                            stream
                                .write(
                                    format!("Bitcoin Address: {}\n", record.bitcoin_address)
                                        .as_bytes(),
                                )
                                .unwrap();
                        }
                    }
                }
                "gender" => {
                    for record in records {
                        if record.gender != String::from("") && re.is_match(record.gender.as_str())
                        {
                            stream
                                .write(format!("Gender: {}\n", record.gender).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "ip_address" => {
                    for record in records {
                        if record.ip_address != String::from("")
                            && re.is_match(record.ip_address.as_str())
                        {
                            stream
                                .write(format!("Ip Address: {}\n", record.ip_address).as_bytes())
                                .unwrap();
                        }
                    }
                }
                "organization" => {
                    for record in records {
                        if record.organization != String::from("")
                            && re.is_match(record.organization.as_str())
                        {
                            stream
                                .write(
                                    format!("Organization: {}\n", record.organization).as_bytes(),
                                )
                                .unwrap();
                        }
                    }
                }
                _ => {
                    stream.write("invalid column\n".as_bytes()).unwrap();
                }
            },
            None => {
                stream
                    .write("Please provide a column\n".as_bytes())
                    .unwrap();
            }
        },
        None => {
            stream.write("smth bad happened\n".as_bytes()).unwrap();
        }
    }
}

pub fn handle_client(mut stream: TcpStream, records: &Vec<Record>) -> Result<(), Error> {
    let mut buf = [0; 512];
    loop {
        let bytes_read = stream.read(&mut buf).unwrap();
        if bytes_read == 0 {
            return Ok(());
        }
        let input = String::from_utf8_lossy(&buf[..bytes_read]);
        if input.starts_with("SelectColumn") {
            handle_query(input, &mut stream, records);
        } else if input.starts_with("SelectFromColumn") {
            handle_regex_query(input, &mut stream, records);
        } else {
            stream.write("Invalid command\n".as_bytes()).unwrap();
        }
    }
}

pub fn get_access_token(endpoints: &HashMap<String, Endpoint>) -> Token {
    let localhost = "http://localhost:5000";
    if let Some(url) = endpoints.get(&String::from("register")) {
        let register_url = url;
        let body = reqwest::blocking::get(format!("{}{}", localhost, register_url.link).as_str())
            .unwrap()
            .text()
            .unwrap();
        let access_token = serde_json::from_str(&body.as_str()).unwrap();
        return access_token;
    } else {
        panic!("Could not retrieve access token");
    }
}

pub fn get_home_routes(
    endpoints: &HashMap<String, Endpoint>,
    access_token: &Token,
) -> HomeEndpoint {
    let localhost = "http://localhost:5000";
    if let Some(url) = endpoints.get(&String::from("home")) {
        let client = reqwest::blocking::Client::new();
        let url = format!("{}{}", localhost, url.link.clone());
        let response = make_request(client, url, access_token.access_token.clone());
        let home_endpoints: HomeEndpoint =
            serde_json::from_str(&response.unwrap().as_str()).unwrap();
        return home_endpoints;
    } else {
        panic!("Couldn't access home route");
    }
}

pub fn deserialize_data(data_endpoints: &MutexGuard<VecDeque<DataEndpoint>>) -> Vec<Record> {
    let mut records: Vec<Record> = vec![];
    for record in data_endpoints.iter() {
        let mut record_data = record.data.clone();
        println!("{:?}", record_data);

        if record_data.ends_with("},]\n") {
            record_data.remove(record_data.len() - 3);
        }
        if record.mime_type == String::from("") {
            let mut new_records: Vec<Record> = serde_json::from_str(&record_data.as_str()).unwrap();
            records.append(&mut new_records);
        } else if record.mime_type == String::from("application/xml") {
            record_data = record_data.replace("<dataset>\n", "");
            record_data = record_data.replace("</dataset>\n", "");
            let mut new_records: Vec<Record> =
                serde_xml_rs::from_str(&record_data.as_str()).unwrap();
            records.append(&mut new_records);
        } else if record.mime_type == String::from("text/csv") {
            let mut reader = csv::Reader::from_reader(record_data.as_str().as_bytes());
            let mut deserializer = reader.deserialize();
            while let Some(result) = deserializer.next() {
                let record: Record = result.unwrap();
                records.push(record);
            }
        } else if record.mime_type == String::from("application/x-yaml") {
            let mut new_records: Vec<Record> = serde_yaml::from_str(&record_data.as_str()).unwrap();
            records.append(&mut new_records);
        }
    }
    return records;
}
