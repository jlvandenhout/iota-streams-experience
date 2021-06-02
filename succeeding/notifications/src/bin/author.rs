use iota_streams::{
    app::transport::tangle::{
        PAYLOAD_BYTES,
        client::Client,
    },
    app_channels::api::tangle::Author
};

#[tokio::main]
async fn main() {
    // Get the Seed from the command line
    let args : Vec<String> = std::env::args().collect();
    let seed = args[1].as_str();

    // Connect to an IOTA Node
    let client = Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com");

    // Create the Author
    let encoding = "utf-8";
    let multi_branching = true;
    let mut author = Author::new(seed, encoding, PAYLOAD_BYTES, multi_branching, client);

    // Announce the Channel and get the Channel Address and Announcement Message ID
    let (application_instance, announcement_id) = notifications::announce(&mut author).await;

    // Send the notification
    let notification = "NOTIFICATION".to_string();
    notifications::send(&mut author, &application_instance, &announcement_id, &notification).await;

    // Share the Channel Address and Announcement Message ID with the Recipient
    println!("Now use the Recipient to listen to the Channel and receive the notification, by running:");
    println!("cargo run --bin recipient {} {} <SEED>", application_instance, announcement_id);
}