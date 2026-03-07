use crate::helloasso::HelloAsso;

mod helloasso;

#[tokio::main]
async fn main() {
    let client_id = "7cd522dd834e49b5bc26c6efd0cf4293";
    let client_secret = "y8QjUzceThp2jrriEs0rsutFB2tfITML";
    let organization_slug = "comite-d-interets-de-quartier-du-panier-c-i-q-panier";

    let client = HelloAsso::new(
        client_id.into(),
        client_secret.into(),
        organization_slug.into(),
    )
    .await;

    dbg!(client);
}
