#[tokio::main]
async fn main() {
    // Get the Seed, Channel Address and Announcement Message ID from the command line
    let args : Vec<String> = std::env::args().collect();
    let application_instance = &args[1];
    let announcement_id = &args[2];
    let seed = &args[3];

    let mut recipient = notifications::Recipient::new(seed);

    // Listen to the Channel using the Channel Address and Announcement Message ID
    recipient.listen(application_instance, announcement_id).await;

    // Receive notifications from the Channel
    let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
    loop {
        interval.tick().await;
        recipient.receive().await;
    }
}