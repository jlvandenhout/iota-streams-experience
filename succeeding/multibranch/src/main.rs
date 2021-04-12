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
    let multi_branching = true;

    let author_seed = utils::random_seed();
    let mut author = Author::new(
        author_seed.as_str(),
        encoding,
        PAYLOAD_BYTES,
        multi_branching,
        Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com"),
    );

    let non_subscriber_seed = utils::random_seed();
    let mut non_subscriber = Subscriber::new(
        non_subscriber_seed.as_str(),
        encoding,
        PAYLOAD_BYTES,
        Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com"),
    );

    let subscriber_seed = utils::random_seed();
    let mut subscriber = Subscriber::new(
        subscriber_seed.as_str(),
        encoding,
        PAYLOAD_BYTES,
        Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com"),
    );

    let late_subscriber_seed = utils::random_seed();
    let mut late_subscriber = Subscriber::new(
        late_subscriber_seed.as_str(),
        encoding,
        PAYLOAD_BYTES,
        Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com"),
    );

    // Announce the Channel and share with the possible Subscribers
    let announcement = author.send_announce().await.unwrap();
    non_subscriber.receive_announcement(&announcement).await.unwrap();
    subscriber.receive_announcement(&announcement).await.unwrap();
    late_subscriber.receive_announcement(&announcement).await.unwrap();

    // Subscribe before the Keyload
    let subscription = subscriber.send_subscribe(&announcement).await.unwrap();
    author.receive_subscribe(&subscription).await.unwrap();

    let keyload = author
        .send_keyload_for_everyone(&announcement)
        .await
        .unwrap();

    // Subscribe after the Keyload
    let late_subscription = late_subscriber.send_subscribe(&announcement).await.unwrap();
    author.receive_subscribe(&late_subscription).await.unwrap();


    // Send messages chained to the Keyload
    let public_payload = Bytes("PRIVATE 1".as_bytes().to_vec());
    let masked_payload = Bytes("PRIVATE 1".as_bytes().to_vec());
    let private = author
        .send_signed_packet(&keyload.0, &public_payload, &masked_payload)
        .await
        .unwrap();

    let public_payload = Bytes("PRIVATE 2".as_bytes().to_vec());
    let masked_payload = Bytes("PRIVATE 2".as_bytes().to_vec());
    author
        .send_signed_packet(&private.0, &public_payload, &masked_payload)
        .await
        .unwrap();

    // Send messages chained to the Announcement
    let public_payload = Bytes("PUBLIC 1".as_bytes().to_vec());
    let masked_payload = Bytes("PUBLIC 1".as_bytes().to_vec());
    let public = author
        .send_signed_packet(&announcement, &public_payload, &masked_payload)
        .await
        .unwrap();

    let public_payload = Bytes("PUBLIC 2".as_bytes().to_vec());
    let masked_payload = Bytes("PUBLIC 2".as_bytes().to_vec());
    author
        .send_signed_packet(&public.0, &public_payload, &masked_payload)
        .await
        .unwrap();


    loop {
        let messages = non_subscriber.fetch_next_msgs().await.await;
        if messages.is_empty() {
            println!("Non Subscriber: No more messages...");
            break;
        }

        for message in messages {
            println!("Non Subscriber: Received message...");
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
        let messages = subscriber.fetch_next_msgs().await.await;
        if messages.is_empty() {
            println!("Subscriber: No more messages...");
            break;
        }

        for message in messages {
            println!("Subscriber: Received message...");
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
        let messages = late_subscriber.fetch_next_msgs().await.await;
        if messages.is_empty() {
            println!("Late Subscriber: No more messages...");
            break;
        }

        for message in messages {
            println!("Late Subscriber: Received message...");
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
