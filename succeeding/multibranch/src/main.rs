#[tokio::main]
async fn main() {
    let mut author = utils::create_author(true);
    let mut non_subscriber = utils::create_subscriber();
    let mut subscriber = utils::create_subscriber();
    let mut late_subscriber = utils::create_subscriber();

    // Announce the Channel and share with the possible Subscribers
    let announcement = author
        .send_announce()
        .await
        .unwrap();
    non_subscriber
        .receive_announcement(&announcement)
        .await
        .unwrap();
    subscriber
        .receive_announcement(&announcement)
        .await
        .unwrap();
    late_subscriber
        .receive_announcement(&announcement)
        .await
        .unwrap();

    // Subscribe before the Keyload
    let subscription = subscriber
        .send_subscribe(&announcement)
        .await
        .unwrap();
    author
        .receive_subscribe(&subscription)
        .await
        .unwrap();

    let keyload = author
        .send_keyload_for_everyone(&announcement)
        .await
        .unwrap();

    // Subscribe after the Keyload
    let late_subscription = late_subscriber
        .send_subscribe(&announcement)
        .await
        .unwrap();
    author
        .receive_subscribe(&late_subscription)
        .await
        .unwrap();


    // Send messages chained to the Keyload
    let public_payload = utils::create_payload("PRIVATE 1");
    let masked_payload = utils::create_payload("PRIVATE 1");
    let private = author
        .send_signed_packet(&keyload.0, &public_payload, &masked_payload)
        .await
        .unwrap();

    let public_payload = utils::create_payload("PRIVATE 2");
    let masked_payload = utils::create_payload("PRIVATE 2");
    author
        .send_signed_packet(&private.0, &public_payload, &masked_payload)
        .await
        .unwrap();

    // Send messages chained to the Announcement
    let public_payload = utils::create_payload("PUBLIC 1");
    let masked_payload = utils::create_payload("PUBLIC 1");
    let public = author
        .send_signed_packet(&announcement, &public_payload, &masked_payload)
        .await
        .unwrap();

    let public_payload = utils::create_payload("PUBLIC 2");
    let masked_payload = utils::create_payload("PUBLIC 2");
    author
        .send_signed_packet(&public.0, &public_payload, &masked_payload)
        .await
        .unwrap();

    utils::receive_messages_for_subscriber(&mut non_subscriber, "Non Subscriber").await;
    utils::receive_messages_for_subscriber(&mut subscriber, "Non Subscriber").await;
    utils::receive_messages_for_subscriber(&mut late_subscriber, "Non Subscriber").await;
}
