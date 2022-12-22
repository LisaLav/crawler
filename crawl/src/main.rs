use reqwest::Client;
use queues::*;

mod frontier;
pub mod dnsresolution;
pub mod fetch;
pub mod parse;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let mut frontQueue = frontier::createFrontQueue();
    println!("{:?}", frontQueue.remove().unwrap());
    println!("{:?}", frontQueue.remove().unwrap());
    
    
    let client = Client::new();
    let mut frontier: Vec<&str> = Vec::new();
    frontier.push("https://www.rust-lang.org/");
    println!("{}", urls.pop().unwrap());
    let resp = client.get(frontier.pop().unwrap()).send()
        .await
        .unwrap()
        .text()
        .await;
    println!("{:#?}", resp);
    Ok(())
}