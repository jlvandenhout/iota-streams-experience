#[tokio::main]
async fn main() {
    let mut author = utils::create_author(false);
    let public_payload = utils::create_payload("PUBLIC MESSAGE");
    let masked_payload = utils::create_payload("MASKED MESSAGE");

    let announcement = author
        .send_announce()
        .await
        .unwrap();

    let packet = author
        .send_signed_packet(&announcement, &public_payload, &masked_payload)
        .await
        .unwrap();
    println!("Message Index: {}", utils::get_hash(&packet.0));

    let keyload = author
        .send_keyload_for_everyone(&announcement)
        .await
        .unwrap();

    let packet = author
        .send_signed_packet(&keyload.0, &public_payload, &masked_payload)
        .await
        .unwrap();
    println!("Message Index: {}", utils::get_hash(&packet.0));
}
