# Notifications
**This tutorial guides us through the process of building a notification service to notify Subscribers using the Streams Channels protocol and the Tangle. At the end of the tutorial we will have an Author, capable of announcing a Channel and sending notifications, and a Subscriber, capable of subscribing to a Channel and receiving notifications.**

>This tutorial is based on the [original Channel tutorial](https://docs.iota.org/docs/channels/1.3/tutorials/build-a-messaging-app), but tailored to Streams 1.0 and IOTA 1.5 (Chrysalis) and providing a solution to some issues the community had using this tutorial.


## Prerequisites
To complete this tutorial, we need the following:
- [Rust](https://www.rust-lang.org/tools/install)
- An editor. We recommend [Visual Studio Code](https://code.visualstudio.com/Download) with the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension


## References
If we're new to Rust, or don't understand something in the code, the following resources might be useful:

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust documentation](https://doc.rust-lang.org/std/) (we can also open the documentation offline with the `rustup doc` command)


## Step 1. Create our project
The best way to start a new project is to use the [Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) build tool, because it handles a lot of tasks for us such as building our code and downloading and building its dependencies.

In this step, we use Cargo to create a new project and list the dependencies. We will create a library project as a base for our Author and Subscriber binaries, which we will create later.

1. Use Cargo to create a new library project. You can replace `notifications` with your own project name if you want.

    ```bash
    cargo new --lib notifications
    ```

    This command creates a new directory with the same name as our project. Inside that directory is a `Cargo.toml` file, which contains our project configuration settings, and a `lib.rs`.

2. Open the `Cargo.toml` file, and add the following dependencies under the `[dependencies]` section

    ```bash
    iota-streams = { git = "https://github.com/iotaledger/streams", branch  = "chrysalis-2", branch = "chrysalis-2", features = ["std", "tangle"] }
    tokio = { version = "1.1" }
    ```

    The `tokio` crate is needed here, because some parts of the Streams crate are asynchronous and need a runtime. This crate provides that runtime.

3. In `lib.rs` list all the functionality we will be using in the following steps.

    ```rust
    use iota_streams::app_channels::api::tangle::{
        Author,
        Address,
        Bytes,
        Subscriber,
        Transport,
        MessageContent,
    };
    ```

Now we have all the dependencies and functionality listed, we're ready to start coding. Our project should now look like this:

```
src/
    lib.rs
Cargo.toml
```


## Step 2. Announce the Channel
In this step, we write a function that announces a new Channel, which Subscribers can subscribe to to receive notifications sent by the Author.

1. In the `lib.rs` file, create a function called `announce`.

    ```rust
    /// Announce the Channel and return the Application Instance and Message ID to
    /// share with Subscribers, so they can subscribe to the Channel.
    pub fn announce<T: Transport>(author: &mut Author<T>) -> (String, String) {

    }
    ```

2. Announce the Channel.

    ```rust
        let announcement_link = author
            .send_announce()
            .expect("Failed to announce the Channel");

        println!("Announced the Channel");
    ```

3. Return the Application Instance and Message ID to share with the Subscribers.

    ```rust
        (announcement_link.appinst.to_string(), announcement_link.msgid.to_string())
    ```

    As an Author, we must send these Link details to any Subscriber who wants to receive our notifications. This can be done by any means. In this tutorial, we'll do this by passing them to the Subscriber via command line arguments.


## Step 3. Send a notification
In this step, we write a function that sends a notification to the Channel. The message will be signed and public to allow all Subscribers to read it and verify that it was the Author who sent it.

1. Create a function called `send` that takes the Link details and the notification to send.

    ```rust
    /// Send a notification to the Channel. This notification message is linked to
    /// the announcement message, so Subscribers are able to find it.
    pub fn send<T: Transport>(
        author: &mut Author<T>,
        application_instance: &String,
        announcement_id: &String,
        notification: &String,
    ) {

    }
    ```

2. Convert the Application Instance and Message ID to a Link to link the notification message to.

    ```rust
        let announcement_link = Address::from_str(&application_instance,&announcement_id)
            .expect("Failed to create the Announcement Link");
    ```

3. Convert the notification to bytes, as that is the only thing the Streams protocol cares about.

    ```rust
        let public_payload = Bytes(notification.as_bytes().to_vec());
        let masked_payload = Bytes("".as_bytes().to_vec());
    ```

    In this case we only use the public payload. The masked payload is used when we only want certain Subscribers to be able to read the data. For that to work, we would first need to send a Keyload message.

4. Send the message, signed by the Author, using the Link and the payloads.

    ```rust
        author
            .send_signed_packet(&announcement_link, &public_payload, &masked_payload)
            .expect("Failed to send the notification to the Channel");

        println!("Sent the notification to the Channel");
    ```


## Step 4. Create the Author
In this step, we will create the Author binary that uses the functions we just created to announce a Channel and send a notification to it.

1. In the `src` folder, create a subfolder named `bin`. Any modules in this folder will automatically be compiled to separate binaries by Cargo. In this folder create a file called `author.rs`.

    Our project should now look like this:

    ```
    src/
        bin/
            author.rs
        lib.rs
    Cargo.toml
    ```

2. In `author.rs`, list the functionality we will be using.

    ```rust
    use iota_streams::{
        app::transport::tangle::{
            PAYLOAD_BYTES,
            client::Client,
        },
        app_channels::api::tangle::Author
    };
    ```

3. Create an asynchronous main function using `tokio` and grab the Author Seed from the command line.

    ```rust
    #[tokio::main]
    async fn main() {
        // Get the Seed from the command line
        let args : Vec<String> = std::env::args().collect();
        let seed = args[1].as_str();

    }
    ```

4. Create an Author using an IOTA Client.

    ```rust
        // Connect to an IOTA Node
        let client = Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com");

        // Create the Author
        let encoding = "utf-8";
        let multi_branching = true;
        let mut author = Author::new(seed, encoding, PAYLOAD_BYTES, multi_branching, client);
    ```

    As the Seed is the key to our Channel, it should always be kept a secret.

5. Now we are ready to announce the Channel and send a notification using our library functions.

    ```rust
        // Announce the Channel and get the Channel Address and Announcement Message ID
        let (application_instance, announcement_id) = notifications::announce(&mut author);

        // Send the notification
        let notification = "NOTIFICATION".to_string();
        notifications::send(&mut author, &application_instance, &announcement_id, &notification);
    ```

6. For our convenience, let's print the command to run the Subscriber to the console:

    ```rust
        // Share the Channel Address and Announcement Message ID with the Subscriber
        println!("Now use the Subscriber to subscribe to the Channel and receive the notification, by running:");
        println!("cargo run --bin subscriber <SEED> {} {}", application_instance, announcement_id);
    ```

Now that the Author has announced the Channel, shared the Link details and sent a notification, let's code the functionality for our Subscriber to subscribe to the Channel and receive the notification.


## Step 5. Subscribe to the Channel
In this step, we write a function that subscribes the Subscriber to the Channel, using the Application Instance and Message ID.

1. In the `lib.rs` file, create a function called `subscribe`.

    ```rust
    /// Subscribe to a Channel using the Application Instance and Message ID
    /// shared by the Author.
    pub fn subscribe<T: Transport>(
        subscriber: &mut Subscriber<T>,
        application_instance: &String,
        announcement_id: &String,
    ) {

    }
    ```

2. Convert the Application Instance and Message ID to a Link to subscribe to.

    ```rust
        let announcement_link = Address::from_str(&application_instance,&announcement_id)
            .expect("Failed to create the Announcement Link");
    ```

2. Subscribe to the Channel.

    ```rust
        subscriber
            .receive_announcement(&announcement_link)
            .expect("Failed to subscribe to the Channel");

        println!("Subscribed to the Channel");
    ```


## Step 6. Receive a notification
In this step, we write a function that receives a notification from the Channel. The message will be verified and converted from bytes to plain text.

1. Create a function called `receive`.

    ```rust
    /// Receive any notifications sent by the Author to the Channel.
    pub fn receive<T: Transport>(subscriber: &mut Subscriber<T>) {

    }
    ```

2. Fetch any messages pending for our Subscriber.

    ```rust
        let messages = subscriber.fetch_next_msgs();
    ```

3. Check if any messages are received and, if so, convert the public payload to plain text and print.

    ```rust
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
    ```

    Note that we are only interested int the Signed Packet messages for now. There are other messages types we could listen to, like Keyload messages.


## Step 7. Create the Subscriber
In this step, we will create the Subscriber binary that uses the functions we just created to subscribe to a Channel and receive any notifications sent to it by the Author.

1. In our `src/bin` folder create a file called `subscriber.rs`.

    Our project should now look like this:

    ```
    src/
        bin/
            author.rs
            subscriber.rs
        lib.rs
    Cargo.toml
    ```

2. In `subscriber.rs`, list the functionality we will be using.

    ```rust
    use iota_streams::{
        app::transport::tangle::{
            PAYLOAD_BYTES,
            client::Client,
        },
        app_channels::api::tangle::Subscriber
    };
    ```

3. Create an asynchronous main function using `tokio` and grab the Subscriber Seed, Application Instance and Message ID from the command line.

    ```rust
    #[tokio::main]
    async fn main() {
        // Get the Seed, Channel Address and Announcement Message ID from the command line
        let args : Vec<String> = std::env::args().collect();
        let seed = args[1].as_str();
        let application_instance = &args[2];
        let announcement_id = &args[3];

    }
    ```

4. Create a Subscriber using an IOTA Client.

    ```rust
        // Connect to an IOTA Node
        let client = Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com");

        // Create the Subscriber
        let encoding = "utf-8";
        let mut subscriber = Subscriber::new(seed, encoding, PAYLOAD_BYTES, client);
    ```

    As with the Seed of the Author, the Seed of the Subscriber should be kept secret.

5. Now we are ready to subscribe to the Channel and receive notifications using our library functions.

    ```rust
        // Subscribe to the Channel using the Channel Address and Announcement Message ID
        notifications::subscribe(&mut subscriber, application_instance, announcement_id);

        // Receive notifications from the Channel
        notifications::receive(&mut subscriber);
    ```


We now have an Author and a Subscriber to create and consume our notification service. From the root of this crate use the following commands to run the Author and the Subscriber:

```bash
cargo run --bin author <SEED>
cargo run --bin subscriber <SEED> <APPLICATION_INSTANCE> <MESSAGE_ID>
```