use iota_streams::app_channels::api::tangle::{
    Author,
    Address,
    Bytes,
    Subscriber,
    Transport,
    MessageContent,
};


/// Announce the Channel and return the Application Instance and Message ID to
/// share with Recipients, so they can listen to the Channel.
pub async fn announce<T: Transport>(author: &mut Author<T>) -> (String, String) {
    let announcement_link = author
        .send_announce()
        .await
        .expect("Failed to announce the Channel");

    println!("Announced the Channel");
    (announcement_link.appinst.to_string(), announcement_link.msgid.to_string())
}


/// Send a notification to the Channel. This notification message is linked to
/// the announcement message, so Recipients are able to find it.
pub async fn send<T: Transport>(
    author: &mut Author<T>,
    application_instance: &String,
    announcement_id: &String,
    notification: &String,
) {
    let announcement_link = Address::from_str(&application_instance,&announcement_id)
        .expect("Failed to create the Announcement Link");

    let public_payload = Bytes(notification.as_bytes().to_vec());
    let masked_payload = Bytes("".as_bytes().to_vec());
    author
        .send_signed_packet(&announcement_link, &public_payload, &masked_payload)
        .await
        .expect("Failed to send the notification to the Channel");

    println!("Sent the notification to the Channel");
}


/// Listen to a Channel using the Application Instance and Message ID
/// shared by the Author.
pub async fn listen<T: Transport>(
    recipient: &mut Subscriber<T>,
    application_instance: &String,
    announcement_id: &String,
) {
    let announcement_link = Address::from_str(&application_instance,&announcement_id)
        .expect("Failed to create the Announcement Link");

    recipient
        .receive_announcement(&announcement_link)
        .await
        .expect("Failed to listen to the Channel");

    println!("Listening to the Channel");
}


/// Receive any notifications sent by the Author to the Channel.
pub async fn receive<T: Transport>(recipient: &mut Subscriber<T>) {
    let messages = recipient.fetch_next_msgs().await;

    if messages.is_empty() {
        println!("No notifications");
    } else {
        for message in messages {
            match message.body {
                MessageContent::SignedPacket { pk: _, public_payload, masked_payload: _ } => {
                    println!("Notification: {}", String::from_utf8(public_payload.0).unwrap());
                },
                _ => {}
            }
        }
    }
}