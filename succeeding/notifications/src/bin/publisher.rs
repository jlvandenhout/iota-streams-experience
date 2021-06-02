use std::io::{
    BufRead,
    stdin,
};

#[tokio::main]
async fn main() {
    // Get the Seed from the command line
    let args : Vec<String> = std::env::args().collect();
    let seed = args[1];

    let publisher = notifications::Publisher::new(seed);

    // Announce the Channel and get the Channel Address and Announcement Message ID
    let (application_instance, mut message_id) = publisher.announce().await;

    // Share the Channel Address and Announcement Message ID with the Recipient
    println!("Now open another terminal and run:");
    println!("cargo run --bin recipient {} {} <RANDOM_SEED>", application_instance, message_id);
    println!("Send a notification:");

    // Send notifications
    for line in stdin().lock().lines() {
        let notification = line.expect("Unable to read notification");

        println!("Sending the notification");
        message_id = publisher.publish(&message_id, &notification).await;
        println!("Send another notification:");
    }
}