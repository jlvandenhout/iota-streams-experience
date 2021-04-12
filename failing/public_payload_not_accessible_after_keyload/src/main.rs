use iota_streams::{
    app::transport::tangle::{
        PAYLOAD_BYTES,
        client::Client,
    },
    app_channels::api::tangle::{
        Author,
        Bytes,
        MessageContent,
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

    let subscriber_before_keyload_seed = utils::random_seed();
    let mut subscriber_before_keyload = Subscriber::new(
        subscriber_before_keyload_seed.as_str(),
        encoding,
        PAYLOAD_BYTES,
        Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com"),
    );

    let subscriber_after_keyload_seed = utils::random_seed();
    let mut subscriber_after_keyload = Subscriber::new(
        subscriber_after_keyload_seed.as_str(),
        encoding,
        PAYLOAD_BYTES,
        Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com"),
    );

    let announcement = author.send_announce().unwrap();
    subscriber_before_keyload.receive_announcement(&announcement).unwrap();
    subscriber_after_keyload.receive_announcement(&announcement).unwrap();

    let subscription = subscriber_before_keyload.send_subscribe(&announcement).unwrap();
    author.receive_subscribe(&subscription).unwrap();

    let public_payload = Bytes("PUBLIC MESSAGE".as_bytes().to_vec());
    let masked_payload = Bytes("MASKED MESSAGE".as_bytes().to_vec());
    let packet = author
        .send_signed_packet(&announcement, &public_payload, &masked_payload)
        .unwrap();
    println!("Message Index: {}", utils::get_hash(&packet.0));

    let keyload = author.send_keyload_for_everyone(&packet.0).unwrap();

    let subscription = subscriber_after_keyload.send_subscribe(&announcement).unwrap();
    author.receive_subscribe(&subscription).unwrap();

    let public_payload = Bytes("PUBLIC MESSAGE".as_bytes().to_vec());
    let masked_payload = Bytes("MASKED MESSAGE".as_bytes().to_vec());
    let packet = author
        .send_signed_packet(&keyload.0, &public_payload, &masked_payload)
        .unwrap();
    println!("Message Index: {}", utils::get_hash(&packet.0));


    loop {
        let messages = subscriber_before_keyload.fetch_next_msgs().await;
        if messages.is_empty() {
            println!("subscriber_before_keyload: No more messages...");
            break;
        }

        for message in messages {
            println!("subscriber_before_keyload: Received message...");
            println!("    Message Index: {}", utils::get_hash(&message.link));
            println!("    Message ID: {}", message.link.msgid);
            match message.body {
                MessageContent::SignedPacket { pk: _, public_payload, masked_payload } => {
                    println!("    Public Message: {}", String::from_utf8(public_payload.0).unwrap());
                    println!("    Masked Message: {}", String::from_utf8(masked_payload.0).unwrap());
                },
                MessageContent::Keyload => {
                    println!("    Keyload");
                },
                _ => {}
            }
        }
    }

    loop {
        let messages = subscriber_after_keyload.fetch_next_msgs().await;
        if messages.is_empty() {
            println!("subscriber_after_keyload: No more messages...");
            break;
        }

        for message in messages {
            println!("subscriber_after_keyload: Received message...");
            println!("    Message Index: {}", utils::get_hash(&message.link));
            println!("    Message ID: {}", message.link.msgid);
            match message.body {
                MessageContent::SignedPacket { pk: _, public_payload, masked_payload } => {
                    println!("    Public Message: {}", String::from_utf8(public_payload.0).unwrap());
                    println!("    Masked Message: {}", String::from_utf8(masked_payload.0).unwrap());
                },
                MessageContent::Keyload => {
                    println!("    Keyload");
                },
                _ => {}
            }
        }
    }
}
