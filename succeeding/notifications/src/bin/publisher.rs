use std::io::BufRead;


#[tokio::main]
async fn main() {
    // Get the Seed from the command line
    let args : Vec<String> = std::env::args().collect();
    let seed = &args[1];

    let mut publisher = notifications::Publisher::new(seed);

    // Announce the Channel and get the Application Instance and Announcement Message ID
    let (application_instance, mut message_id) = publisher.announce().await;

    // Share the Application Instance and Announcement Message ID with the Recipient
    println!("Now open another terminal and run:");
    println!("cargo run --bin recipient {} {} <RANDOM_SEED>", application_instance, message_id);
    println!("Send a notification:");

    // Send notifications
    for line in std::io::stdin().lock().lines() {
        let notification = line.expect("Unable to read the notification");

        message_id = publisher.publish(&application_instance, &message_id, &notification).await;
        println!("Send another notification:");
    }
}