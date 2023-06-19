use std::collections::HashMap;

extern crate reqwest;

pub fn register_client(host: &str, uid: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut obj = HashMap::new();
    obj.insert("uid", uid);

    let client = reqwest::blocking::Client::new();
    let res = client.post(host).json(&obj).send();
    println!("{:?}", res?);
    return Ok(());
}
