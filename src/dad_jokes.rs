#[allow(unused)]

use reqwest::header::ACCEPT;
use std::io::Read;
use json;

pub fn get() -> String {
    let client = reqwest::blocking::Client::new();
    let mut response = client.get("https://icanhazdadjoke.com/").header(ACCEPT, "application/json").send().unwrap();
    let mut content = String::new();
    response.read_to_string(&mut content);
    let parsed = json::parse(&content).unwrap();
    String::from(parsed["joke"].to_string())
}
