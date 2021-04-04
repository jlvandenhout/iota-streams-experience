use iota_streams::{
    app::transport::tangle::{
        PAYLOAD_BYTES,
        client::Client,
    },
    app_channels::api::tangle::{
        Author,
        Bytes,
    },
};

#[tokio::main]
async fn main() {
    let encoding = "utf-8";
    let multi_branching = false;

    let author_seed = utils::random_seed();
    let mut author = Author::new(
        author_seed.as_str(),
        encoding,
        PAYLOAD_BYTES,
        multi_branching,
        Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com"),
    );

    let announcement = author.send_announce().await.unwrap();

    let public_payload = Bytes("PUBLIC MESSAGE".as_bytes().to_vec());
    let masked_payload = Bytes("MASKED MESSAGE".as_bytes().to_vec());
    let packet = author
        .send_signed_packet(&announcement, &public_payload, &masked_payload)
        .await
        .unwrap();
    println!("Message Index: {}", utils::get_hash(&packet.0));

    let keyload = author.send_keyload_for_everyone(&announcement).await.unwrap();

    let public_payload = Bytes("PUBLIC MESSAGE".as_bytes().to_vec());
    let masked_payload = Bytes("MASKED MESSAGE".as_bytes().to_vec());
    let packet = author
        .send_signed_packet(&keyload.0, &public_payload, &masked_payload)
        .await
        .unwrap();
    println!("Message Index: {}", utils::get_hash(&packet.0));
}
