use dioxus::prelude::*;
use arti_client::{TorClient, TorClientConfig};
use futures::executor::block_on;

fn main() {
    dioxus::mobile::launch(app);
}

fn app() -> Element {
    // Run async Tor client bootstrap and connection once at app start
    // Here we block on it for simplicity; in a real app, use async properly.
    block_on(run_tor_client());

    rsx! {
        div { "Hello from Dioxus mobile frontend!" }
    }
}

async fn run_tor_client() {
    // Default Tor client configuration
    let config = TorClientConfig::default();

    // Create and bootstrap the Tor client
    match TorClient::create_bootstrapped(config).await {
        Ok(tor_client) => {
            // Connect to example.com:80 anonymously over Tor
            match tor_client.connect(("example.com", 80)).await {
                Ok(mut stream) => {
                    println!("Connected to example.com over Tor!");
                    // You can now read/write from/to `stream` (AsyncRead/AsyncWrite)
                }
                Err(e) => {
                    eprintln!("Failed to connect over Tor: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to bootstrap Tor client: {}", e);
        }
    }
}
