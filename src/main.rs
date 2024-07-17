use bobadojo_apis::bobadojo::stores::v1::stores_client::StoresClient;
use bobadojo_apis::bobadojo::stores::v1::ListStoresRequest;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};
use tonic::Request;

const ENDPOINT: &str = "https://stores-ryzkosl3jq-uw.a.run.app:443";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = std::path::PathBuf::from_iter(["data"]);
    let pem = std::fs::read_to_string(data_dir.join("tls/ca.pem"))?;
    let ca = Certificate::from_pem(pem);

    let tls = ClientTlsConfig::new()
        .ca_certificate(ca)
        .domain_name("googleapis.com");

    let channel = Channel::from_static(ENDPOINT)
        .tls_config(tls)?
        .connect()
        .await?;

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
