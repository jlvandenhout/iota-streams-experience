use iota_streams::{
    app::transport::tangle::{
        client::Client,
        PAYLOAD_BYTES,
    },
    app_channels::api::tangle::{
        Author,
        Address,
        Bytes,
        Subscriber,
        MessageContent,
    },
};

pub struct Publisher {
    inner: Author<Client>,
}


impl Publisher {
    pub fn new(seed: &String) -> Self {
        Self {
            inner: Author::new(
                seed,
                "utf-8",
                PAYLOAD_BYTES,
                false,
                Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com")
            )
        }
    }

    /// Announce the Channel and return the Application Instance and Message ID to
    /// share with Recipients, so they can listen to the Channel.
    pub async fn announce(&mut self) -> (String, String) {
        let link = self.inner
            .send_announce()
            .await
            .expect("Failed to announce the Channel");

        (link.appinst.to_string(), link.msgid.to_string())
    }

    /// Publish a notification to the Channel. This notification message is linked to
    /// a previous message, so Recipients are able to find it.
    pub async fn publish(
        &mut self,
        application_instance: &String,
        message_id: &String,
        notification: &String,
    ) -> String {
        let link = Address::from_str(application_instance,message_id)
            .expect("Failed to create the Link");

        let public_payload = Bytes(notification.as_bytes().to_vec());
        let masked_payload = Bytes("".as_bytes().to_vec());
        let (link, _) = self.inner
            .send_signed_packet(&link, &public_payload, &masked_payload)
            .await
            .expect("Failed to publish the notification to the Channel");

        link.msgid.to_string()
    }
}

pub struct Recipient {
    inner: Subscriber<Client>,
}


impl Recipient {
    pub fn new(seed: &String) -> Self {
        Self {
            inner: Subscriber::new(
                seed,
                "utf-8",
                PAYLOAD_BYTES,
                Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com"),
            )
        }
    }

    /// Listen to a Channel using the Application Instance and Message ID
    /// shared by the Publisher.
    pub async fn listen(
        &mut self,
        application_instance: &String,
        announcement_id: &String,
    ) {
        let announcement_link = Address::from_str(&application_instance,&announcement_id)
            .expect("Failed to create the Link");

        self.inner
            .receive_announcement(&announcement_link)
            .await
            .expect("Failed to listen to the Channel");
    }

    /// Receive any notifications sent by the Publisher to the Channel.
    pub async fn receive(&mut self) {
        for message in self.inner.fetch_next_msgs().await {
            if let MessageContent::SignedPacket { pk: _, public_payload, masked_payload: _ } = message.body {
                println!("{}", String::from_utf8(public_payload.0).unwrap());
            }
        }
    }
}