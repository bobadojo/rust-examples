use tonic::{transport::Channel, Request};

use bobadojo_apis::bobadojo::stores::v1::stores_client::StoresClient;
use bobadojo_apis::bobadojo::stores::v1::ListStoresRequest;

const ENDPOINT: &str = "http://localhost:8080";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = Channel::from_static(ENDPOINT).connect().await?;
    let mut service = StoresClient::with_interceptor(channel, move |req: Request<()>| Ok(req));
    let response = service
        .list_stores(Request::new(ListStoresRequest {
            ..Default::default()
        }))
        .await?;
    let mut i = 0;
    for store in response.get_ref().stores.iter() {
        println!("{} stores={:?}", i, store);
        i += 1;
    }
    Ok(())
}
