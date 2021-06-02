use iota_streams::{
    app::transport::tangle::{
        PAYLOAD_BYTES,
        client::Client,
    },
    app_channels::api::tangle::Subscriber
};

#[tokio::main]
async fn main() {
    // Get the Seed, Channel Address and Announcement Message ID from the command line
    let args : Vec<String> = std::env::args().collect();
    let application_instance = &args[1];
    let announcement_id = &args[2];
    let seed = &args[3];

    // Connect to an IOTA Node
    let client = Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com");

    // Create the Recipient
    let encoding = "utf-8";
    let mut recipient = Subscriber::new(seed, encoding, PAYLOAD_BYTES, client);

    // Listen to the Channel using the Channel Address and Announcement Message ID
    notifications::listen(&mut recipient, application_instance, announcement_id).await;

    // Receive notifications from the Channel
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
    loop {
        interval.tick().await;
        notifications::receive(&mut recipient).await;
    }
}