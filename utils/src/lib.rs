use crypto::hashes::{Digest, blake2b};
use iota_streams::{
    app::transport::tangle::{
        PAYLOAD_BYTES,
        client::Client,
    },
    app_channels::api::tangle::{
        Address,
        Author,
        Bytes,
        MessageContent,
        Subscriber,
        Transport,
    }
};
use rand::{
    distributions::Uniform,
    Rng,
    thread_rng,
};


pub fn random_seed() -> String {
    let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ9".as_bytes();
    thread_rng()
        .sample_iter(Uniform::new(0, alphabet.len()))
        .take(81)
        .map(|i| alphabet[i] as char)
        .collect()
}


pub fn get_hash(link: &Address) ->  String {
    let total = [link.appinst.as_ref(), link.msgid.as_ref()].concat();
    let hash = blake2b::Blake2b256::digest(&total);
    hex::encode(&hash)
}


pub fn create_subscriber() -> Subscriber<Client> {
    Subscriber::new(
        random_seed().as_str(),
        "utf-8",
        PAYLOAD_BYTES,
        Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com"),
    )
}


pub fn create_author(multi_branching: bool) -> Author<Client> {
    Author::new(
        random_seed().as_str(),
        "utf-8",
        PAYLOAD_BYTES,
        multi_branching,
        Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com"),
    )
}


pub async fn receive_messages_for_author<T: Transport>(author: &mut Subscriber<T>, tag: &str) {
    println!("");
    println!("Receiving messages for {}...", tag);
    loop {
        let messages = author.fetch_next_msgs().await;
        if messages.is_empty() { break; }

        for message in messages {
            println!("Message Index: {}", get_hash(&message.link));
            println!("Message ID: {}", message.link.msgid);
            match message.body {
                MessageContent::Announce => {
                    println!("    Announcement");
                },
                MessageContent::Keyload => {
                    println!("    Keyload");
                },
                MessageContent::Sequence => {
                    println!("    Sequence");
                },
                MessageContent::SignedPacket { pk: _, public_payload, masked_payload } => {
                    println!("    Public Message: {}", String::from_utf8(public_payload.0).unwrap());
                    println!("    Masked Message: {}", String::from_utf8(masked_payload.0).unwrap());
                },
                MessageContent::Subscribe => {
                    println!("    Subscription");
                },
                MessageContent::TaggedPacket { public_payload, masked_payload } => {
                    println!("    Public Message: {}", String::from_utf8(public_payload.0).unwrap());
                    println!("    Masked Message: {}", String::from_utf8(masked_payload.0).unwrap());
                },
                MessageContent::Unsubscribe => {
                    println!("    Unsubscribtion");
                },
            }
        }
    }
}


pub async fn receive_messages_for_subscriber<T: Transport>(subscriber: &mut Subscriber<T>, tag: &str) {
    println!("");
    println!("Receiving messages for {}...", tag);
    loop {
        let messages = subscriber.fetch_next_msgs().await;
        if messages.is_empty() { break; }

        for message in messages {
            println!("Message Index: {}", get_hash(&message.link));
            println!("Message ID: {}", message.link.msgid);
            match message.body {
                MessageContent::Announce => {
                    println!("    Announcement");
                },
                MessageContent::Keyload => {
                    println!("    Keyload");
                },
                MessageContent::Sequence => {
                    println!("    Sequence");
                },
                MessageContent::SignedPacket { pk: _, public_payload, masked_payload } => {
                    println!("    Public Message: {}", String::from_utf8(public_payload.0).unwrap());
                    println!("    Masked Message: {}", String::from_utf8(masked_payload.0).unwrap());
                },
                MessageContent::Subscribe => {
                    println!("    Subscription");
                },
                MessageContent::TaggedPacket { public_payload, masked_payload } => {
                    println!("    Public Message: {}", String::from_utf8(public_payload.0).unwrap());
                    println!("    Masked Message: {}", String::from_utf8(masked_payload.0).unwrap());
                },
                MessageContent::Unsubscribe => {
                    println!("    Unsubscribtion");
                },
            }
        }
    }
}


pub fn create_payload(payload: &str) -> Bytes {
    Bytes(payload.as_bytes().to_vec())
}