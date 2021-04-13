#[tokio::main]
async fn main() {
    let mut author = utils::create_author(true);
    let mut subscriber_before_keyload = utils::create_subscriber();
    let mut subscriber_after_keyload = utils::create_subscriber();
    let public_payload = utils::create_payload("PUBLIC MESSAGE");
    let masked_payload = utils::create_payload("MASKED MESSAGE");

    let announcement = author
        .send_announce()
        .await
        .unwrap();
    subscriber_before_keyload
        .receive_announcement(&announcement)
        .await
        .unwrap();
    subscriber_after_keyload
        .receive_announcement(&announcement)
        .await
        .unwrap();

    let subscription = subscriber_before_keyload
        .send_subscribe(&announcement)
        .await
        .unwrap();
    author
        .receive_subscribe(&subscription)
        .await
        .unwrap();

    let packet = author
        .send_signed_packet(&announcement, &public_payload, &masked_payload)
        .await
        .unwrap();
    println!("Message Index: {}", utils::get_hash(&packet.0));

    let keyload = author
        .send_keyload_for_everyone(&packet.0)
        .await
        .unwrap();

    let subscription = subscriber_after_keyload
        .send_subscribe(&announcement)
        .await
        .unwrap();
    author
        .receive_subscribe(&subscription)
        .await
        .unwrap();

    let packet = author
        .send_signed_packet(&keyload.0, &public_payload, &masked_payload)
        .await
        .unwrap();
    println!("Message Index: {}", utils::get_hash(&packet.0));

    utils::receive_messages_for_subscriber(&mut subscriber_before_keyload, "Subscriber before Keyload").await;
    utils::receive_messages_for_subscriber(&mut subscriber_after_keyload, "Subscriber after Keyload").await;
}
