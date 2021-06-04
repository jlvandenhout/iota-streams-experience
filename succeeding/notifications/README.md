# Notifications
**This tutorial guides us through the process of building a notification service to notify Recipients using the Streams Channels protocol and the Tangle. At the end of the tutorial we will have a Publisher, capable of announcing a Channel and publishing notifications to it, and one or more Recipients, capable of receiving notifications published to the Channel by the Publisher.**

>This tutorial is based on the [original Channel tutorial](https://docs.iota.org/docs/channels/1.3/tutorials/build-a-messaging-app), but tailored to Streams 1.0 and IOTA 1.5 (Chrysalis) and providing a solution to some issues the community had using this tutorial.


## Prerequisites
To complete this tutorial, we need the following:
- [Rust](https://www.rust-lang.org/tools/install).
- An editor. We recommend [Visual Studio Code](https://code.visualstudio.com/Download) with the [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension.


## References
If you're new to Rust, or don't understand something in the code, the following resources might be useful:

- [Rust Book](https://doc.rust-lang.org/book/).
- [Rust documentation](https://doc.rust-lang.org/std/) (we can also open the documentation offline with the `rustup doc` command).


## Step 1. Create our project
The best way to start a new project is to use the [Cargo](https://doc.rust-lang.org/book/ch01-03-hello-cargo.html) build tool, because it handles a lot of tasks for us such as building our code and downloading and building its dependencies.

In this step, we use Cargo to create a new project and list the dependencies. We will create a library project as a base for our Publisher and Recipient binaries, which we will create later.

1. Use Cargo to create a new library project. You can replace `notifications` with your own project name if you want.

    ```bash
    cargo new --lib notifications
    ```

    This command creates a new directory with the same name as our project. Inside that directory is a `Cargo.toml` file, which contains our project configuration settings, and a `lib.rs`.

2. Open the `Cargo.toml` file, and add the following dependencies under the `[dependencies]` section

    ```bash
    iota-streams = { git = "https://github.com/iotaledger/streams", branch  = "chrysalis-2", branch = "chrysalis-2", features = ["std", "async", "tangle"] }
    tokio = { version = "1.1" }
    ```

    As Streams is inherently asynchronous, it is a lot easier to start using asynchronous features right from the start, so we can await message handling instead of using cumbersome loops to poll if a message is hanled. This means we will use `async fn` in our function definitions and `.await` when we call them to await the result.

    Although Rust comes with asynchronous functionality, it does not provide an asynchronous runtime by default. This means we will use the `tokio` package to provide us with a runtime.

3. In `lib.rs` list all the functionality we will be using in the following steps.

    ```rust
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
    ```

    Now we have all the dependencies and functionality listed, we're ready to start coding. Our project should now look like this:

    ```
    src/
        lib.rs
    Cargo.toml
    ```


## Step 2. Announce the Channel 
In this step, we write some functions in the `lib.rs` file for announcing a new Channel which Recipients can listen to in order to receive notifications published to the Channel by the Publisher.

1. In the `lib.rs` file, create a struct called `Publisher` that encapsulates the `Author` interface. We will use this to build the functions we want our Publisher to expose.

    ```rust
    pub struct Publisher {
        inner: Author<Client>,
    }
    ```

2. In the `lib.rs` file, create an implementation block for the `Publisher` struct.

    ```rust
    impl Publisher {

    }
    ```

3. In the implementation block provide a way to create a new Publisher from a provided seed and connect it to the IOTA Testnet through a public node.

    ```rust
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
    ```

4. Create a function to announce the Channel and return the Application Instance and Message ID to share with Recipients, so they can listen to the Channel.

    ```rust
        /// Announce the Channel and return the Application Instance and Message ID to
        /// share with Recipients, so they can listen to the Channel.
        pub async fn announce(&mut self) -> (String, String) {
            let link = self.inner
                .send_announce()
                .await
                .expect("Failed to announce the Channel");

            (link.appinst.to_string(), link.msgid.to_string())
        }
    ```

    As a Publisher, we must send these Link details (Application Instance and Message ID) to any Recipient who wants to receive our notifications. This can be done by any means. In this tutorial, we'll do this by passing them to the Recipient via command line arguments.


## Step 3. Publish notifications
In this step, we write a function that permits the Publisher to publish notification messages to the Channel. The message will be signed and public to allow all Recipients to read it and verify that it was the Publisher who sent it.

1. In the implementation block of the Publisher create a function called `publish` that takes the Link details and the notification to publish.

    ```rust
        /// Publish a notification to the Channel. This notification message is linked to
        /// a previous message, so Recipients are able to find it.
        pub async fn publish(
            &mut self,
            application_instance: &String,
            message_id: &String,
            notification: &String,
        ) -> String {

        }
    ```

2. In the `publish` function convert the Application Instance and Message ID to a Link to link the notification message to.

    ```rust
            let link = Address::from_str(application_instance,message_id)
                .expect("Failed to create the Link");
    ```

3. Convert the notification to bytes, as that is the only thing the Streams protocol cares about.

    ```rust
            let public_payload = Bytes(notification.as_bytes().to_vec());
            let masked_payload = Bytes("".as_bytes().to_vec());
    ```

    In this case we only use the public payload. The masked payload is used when we only want certain Recipients to be able to read the data. For that to work, we would first need to send a Keyload message.

4. Send the message, signed by the Publisher, using the Link and the payloads and return the Message ID of the sent message so we can link to it later.

    ```rust
            let (link, _) = self.inner
                .send_signed_packet(&link, &public_payload, &masked_payload)
                .await
                .expect("Failed to publish the notification to the Channel");

            link.msgid.to_string()
    ```


## Step 4. Create the Publisher
In this step, we will create the Publisher binary that uses the functions we just created to announce a Channel and publish one or multiple notifications to it.

1. In the `src` folder, create a subfolder named `bin`. Any modules in this folder will automatically be compiled to separate binaries by Cargo. In this folder create a file called `publisher.rs`.

    Our project should now look like this:

    ```
    src/
        bin/
            publisher.rs
        lib.rs
    Cargo.toml
    ```

2. In `publisher.rs`, list the functionality we will be using.

    ```rust
    use std::io::BufRead;
    ```

3. Create an asynchronous main function using `tokio` and grab the seed from the command line.

    ```rust
    #[tokio::main]
    async fn main() {
        // Get the Seed from the command line
        let args : Vec<String> = std::env::args().collect();
        let seed = &args[1];

    }
    ```

4. Create a Publisher by calling the "new" function of the "Publisher" struct contained in lib.rs file.

    ```rust
        let mut publisher = notifications::Publisher::new(seed);
    ```

    As the Seed is the key to our Channel, it should always be kept a secret.

5. Now we that we have a Publisher, we are ready to announce the Channel 

    ```rust
        // Announce the Channel and get the Channel Address and Announcement Message ID
        let (application_instance, mut message_id) = publisher.announce().await;

    ```

6. We now print the command that each recipient will have to execute in order to listen on the channel that the publisher has just created.
NOTE: In a real scenario the Publisher must find an efficient and safe way to communicate Channel Address and Announcement Message ID to each recipient who intends to subscribe to the channel created by the Publisher.

    ```rust
        // Share the Channel Address and Announcement Message ID with the Recipient
        println!("Now open another terminal and run:");
        println!("cargo run --bin recipient {} {} <RANDOM_SEED>", application_instance, message_id);
        println!("Send a notification:");
    ```

6. Now that the Publisher has announced the Channel he can begin to publish notifications on this channel

    ```rust
        // Send notifications
        for line in std::io::stdin().lock().lines() {
            let notification = line.expect("Unable to read the notification");

            message_id = publisher.publish(&application_instance, &message_id, &notification).await;
            println!("Send another notification:");
        }
    ```

Now that the Publisher has announced the Channel, shared the Link details and sent one or more notifications, let's code the functionality for our Recipient to listen to the Channel and receive the notification.


## Step 5. Listen to the Channel
In this step, we write some functions that lets the Recipient listen to the Channel and so receiving notifications, using the Application Instance and Message ID.


1. In the `lib.rs` file, create a scruct called `Recipient` that inherits from `Subscriber`

    ```rust
    pub struct Recipient {
        inner: Subscriber<Client>,
    }
    ```

2. In the `lib.rs` file, create an implementation of the struct `Recipient`.

    ```rust
    impl Recipient {

    }
    ```

3. In the implementation create a Recipient starting from a seed chosen from him and by connecting to the IOTA test network through a public node .

    ```rust
        pub fn new(seed: &String) -> Self {
            Self {
                inner: Subscriber::new(
                    seed,
                    "utf-8",
                    PAYLOAD_BYTES,
                    false,
                    Client::new_from_url("https://api.lb-0.testnet.chrysalis2.com"),
                )
            }
        }
    ```

4. Create a function that listen to a Channel using the Application Instance and Message ID shared by the Publisher.

    ```rust
        /// Listen to a Channel using the Application Instance and Message ID
        /// shared by the Publisher.
        pub async fn listen(
            &mut self,
            application_instance: &String,
            announcement_id: &String,
        ) {

        }
    ```

5. Convert the Application Instance and Message ID to a Link to listen to.

    ```rust
            let announcement_link = Address::from_str(&application_instance,&announcement_id)
                .expect("Failed to create the Link");
    ```

6. Listen to the Channel.

    ```rust
            self.inner
                .receive_announcement(&announcement_link)
                .await
                .expect("Failed to listen to the Channel");
    ```


## Step 6. Receive notifications
In this step, we write a function that receives notifications from the Channel. The messages will be verified and converted from bytes to plain text.

1. Still in the implementation of the Recipient create a function called `receive`.

    ```rust
        /// Receive any notifications sent by the Publisher to the Channel.
        pub async fn receive(&mut self) {
            
        }
    ```


2. Fetch continuosly if there are any messages pending for our Recipient.

    ```rust
            for message in self.inner.fetch_next_msgs().await {
                
            }
    ```

3. Check if any messages are received and, if so, convert the public payload to plain text and print.

    ```rust
                if let MessageContent::SignedPacket { pk: _, public_payload, masked_payload: _ } = message.body {
                    println!("{}", String::from_utf8(public_payload.0).unwrap());
                }
    ```

    Note that we are only interested in the Signed Packet messages for now. There are other messages types we could listen to, like Keyload messages.



## Step 7. Create the Recipient
In this step, we will create the Recipient binary that uses the functions we just created to listen to a Channel and receive any notifications sent to it by the Publisher.

1. In our `src/bin` folder create a file called `recipient.rs`.

    Our project should now look like this:

    ```
    src/
        bin/
            publisher.rs
            recipient.rs
        lib.rs
    Cargo.toml
    ```

2. Create an asynchronous main function using `tokio` and grab respectively Application Instance, Announcement ID and recipient-seed from the command line.

    ```rust
    #[tokio::main]
    async fn main() {
        // Get the Seed, Channel Address and Announcement Message ID from the command line
        let args : Vec<String> = std::env::args().collect();
        let application_instance = &args[1];
        let announcement_id = &args[2];
        let seed = &args[3];

    }
    ```

3. Create a Recipient by calling the "new" function of the "Recipient" struct contained in lib.rs file.

    ```rust
        let mut recipient = notifications::Recipient::new(seed);

    ```

4. Now we are ready to listen the Channel 

    ```rust
        // Listen to the Channel using the Channel Address and Announcement Message ID
        recipient.listen(application_instance, announcement_id).await;

        // Receive notifications from the Channel
        let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));
        loop {
            interval.tick().await;
            recipient.receive().await;
        }
    ```

## Step 9. Run the scripts
We now have an Publisher and a Recipient to create and consume our notification service. From the root of this package use the following commands to run the Publisher and the Recipient:

```bash
cargo run --bin author <SEED>
cargo run --bin recipient <SEED> <APPLICATION_INSTANCE> <MESSAGE_ID>
```
