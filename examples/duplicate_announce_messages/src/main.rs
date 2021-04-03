use iota_streams::{
    app::transport::tangle::{
        PAYLOAD_BYTES,
        client::Client,
    },
    app_channels::api::tangle::{
        Author,
        Subscriber,
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

    let subscriber_seed = utils::random_seed();
    let mut subscriber = Subscriber::new(
        subscriber_seed.as_str(),
        encoding,
        PAYLOAD_BYTES,
        Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com"),
    );

    let announcement = author.send_announce().await.unwrap();
    author.send_announce().await.unwrap();

    subscriber.receive_announcement(&announcement).await.unwrap();
}
