mod helloasso;

#[tokio::main]
async fn main() {
    let client_id = "7cd522dd834e49b5bc26c6efd0cf4293";
    let client_secret = "y8QjUzceThp2jrriEs0rsutFB2tfITML";

    let client = helloasso::api::ClientAPI::new("https://api.helloasso.com").unwrap();
    let token = client
        .request_token(client_id.into(), client_secret.into())
        .await
        .unwrap();

    println!("{token:?}");
}
