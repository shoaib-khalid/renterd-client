use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let worker_uri: &str = "http://localhost:9880";
    let worker_api_prefix: &str = "/api";

    get_state(worker_uri, worker_api_prefix).await?;
    upload_file(worker_uri, "/api/worker", "Texts", "D:\\Fawad_file_renterd1.txt").await?;
    
    Ok(())
}

//Todo build the basic api status method and see if it is able to connect or not

pub async fn upload_file(
    worker_uri: &str,
    worker_api_prefix: &str,
    key: &str,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Open the file
    let mut file = File::open(file_path)?;

    // Read the file into a Vec<u8>
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    // Create a client
    let client = Client::new();

    // Create the URL
    let url = format!(
        "{}{}/objects/PBS-Cloud-Backup/{}",
        worker_uri, worker_api_prefix, key
    );

    // Create the headers
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("text/plain"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_static("Basic OnByb3htb3hpc2dyODJkYXkh"),
    );

    // Send the request
    let res = client.put(&url).headers(headers).body(buf).send().await?;

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

    // Print the URL
    println!("URL: {}", url);

    // Create the headers
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_static("Basic OnByb3htb3hpc2dyODJkYXkh"));

    // Print the headers
    println!("Headers: {:?}", headers);

    // Send the request
    let res = client.get(&url)
        .headers(headers.clone())
        .send()
        .await?;

    // Print the response status
    println!("Response status: {}", res.status());

    // Print the response headers
    println!("Response headers: {:?}", res.headers());

    // Check the status
    if res.status().is_success() {
        // Get the response body as text
        let body = res.text().await?;

        // Print the response body
        println!("Response body: {}", body);
    } else {
        println!("Failed to send request: {}", res.status());
    }

    Ok(())
}

