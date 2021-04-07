use iota_streams::app_channels::api::tangle::Address;

#[tokio::main]
async fn main() {
    // Get the Application Instance and Announcement Message ID from the command line
    let args : Vec<String> = std::env::args().collect();
    let appinst = &args[1];
    let msgid = &args[2];

    // Convert to Link
    let link = Address::from_str(appinst, msgid).unwrap();
    println!("Index: {}", utils::get_hash(&link));
}