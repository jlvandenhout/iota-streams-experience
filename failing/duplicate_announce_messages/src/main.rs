#[tokio::main]
async fn main() {
    let mut author = utils::create_author(false);
    let mut subscriber = utils::create_subscriber();

    let announcement = author
        .send_announce()
        .await
        .unwrap();
    println!("Message Index: {}", utils::get_hash(&announcement));

    let announcement = author
        .send_announce()
        .await
        .unwrap();
    println!("Message Index: {}", utils::get_hash(&announcement));

    subscriber
        .receive_announcement(&announcement)
        .await
        .unwrap();
}
