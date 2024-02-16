use std::fs::File;
use std::io::Read;
use reqwest::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    upload_file("http://localhost:9980", "/api/worker", "Texts", "D:\\Fawad_file_renterd1.txt").await?;
    Ok(())
}

//Todo build the basic api status method and see if it is able to connect or not

pub async fn upload_file(worker_uri: &str, worker_api_prefix: &str, key: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Open the file
    let mut file = File::open(file_path)?;

    // Read the file into a Vec<u8>
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    // Create a client
    let client = Client::new();

    // Create the URL
    let url = format!("{}{}/objects/PBS-Cloud-Backup/{}", worker_uri, worker_api_prefix, key);

    // Create the headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    headers.insert(AUTHORIZATION, HeaderValue::from_static("Basic OnByb3htb3hpc2dyODJkYXkh"));

    // Send the request
    let res = client.put(&url)
        .headers(headers)
        .body(buf)
        .send()
        .await?;

    // Check the status
    if res.status().is_success() {
        println!("File uploaded successfully");
    } else {
        println!("Failed to upload file: {}", res.status());
    }

    Ok(())
}

pub async fn get_state(worker_uri: &str, worker_api_prefix: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create a client
    let client = Client::new();

    // Create the URL
    let url = format!("{}{}/bus/state", worker_uri, worker_api_prefix);

    // Create the headers
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_static("Basic OnByb3htb3hpc2dyODJkYXkh"));

    // Send the request
    let res = client.get(&url)
        .headers(headers)
        .send()
        .await?;

    // Check the status
    if res.status().is_success() {
        println!("Request was successful");
    } else {
        println!("Failed to send request: {}", res.status());
    }

    Ok(())
}
