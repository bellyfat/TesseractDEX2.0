#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate websocket;

use std::thread;
use std::sync::mpsc::channel;
use std::io::stdin;

use websocket::{Message, OwnedMessage};
use websocket::client::ClientBuilder;

// import from our own modules
mod sign_message;
mod request_info;



const CONNECTION: &'static str = "ws://127.0.0.1:2794";

fn main() {

	println!("Connecting to {}", CONNECTION);

	let client = ClientBuilder::new(CONNECTION)
		.unwrap()
		.add_protocol("rust-websocket")
		.connect_insecure()
		.unwrap();

	println!("Successfully connected");

	let (mut receiver, mut sender) = client.split().unwrap();

	let (tx, rx) = channel();

	let tx_1 = tx.clone();

	let send_loop = thread::spawn(move || {
		loop {
			// Send loop
			let message = match rx.recv() {
				Ok(m) => m,
				Err(e) => {
					println!("Send Loop: {:?}", e);
					return;
				}
			};
			match message {
				OwnedMessage::Close(_) => {
					let _ = sender.send_message(&message);
					// If it's a close message, just send it and then return.
					return;
				}
				_ => (),
			}
			// Send the message
			match sender.send_message(&message) {
				Ok(()) => (),
				Err(e) => {
					println!("Send Loop: {:?}", e);
					let _ = sender.send_message(&websocket::Message::close());
					return;
				}
			}
		}
	});

	let receive_loop = thread::spawn(move || {
		// Receive loop
		for message in receiver.incoming_messages() {
			let message = match message {
				Ok(m) => m,
				Err(e) => {
					println!("Receive Loop: {:?}", e);
					let _ = tx_1.send(OwnedMessage::Close(None));
					return;
				}
			};
			match message {
				OwnedMessage::Close(_) => {
					// Got a close message, so send a close message and return
					let _ = tx_1.send(OwnedMessage::Close(None));
					return;
				}
				OwnedMessage::Ping(data) => {
					match tx_1.send(OwnedMessage::Pong(data)) {
						// Send a pong in response
						Ok(()) => (),
						Err(e) => {
							println!("Receive Loop: {:?}", e);
							return;
						}
					}
				}
				// Say what we received
				_ => println!("Receive Loop: {:?}", message),
			}
		}
	});


	loop {
		println!("Please input your request: ");
		let mut input = String::new();
		stdin().read_line(&mut input).unwrap();
		let trimmed = input.trim();
        let json_string = trimmed.to_string();
		
		let request_info: request_info::RequestInfo = serde_json::from_str(&json_string).unwrap();
		println!("{:#?}", request_info);

		if request_info.request_type == "trade" {
			println!("Please input your secret key: ");

			let mut input = String::new();
			
			stdin().read_line(&mut input).unwrap();

			let trimmed = input.trim();

			let sk = trimmed.to_string();
			println!("{}", sk);
			
			let message_to_sign: String = request_info.address.to_string() + &request_info.coin_id.to_string() + &request_info.nonce.to_string();
			sign_message::sign_message(message_to_sign, sk);
		}

		let message = match trimmed {
			"/close" => {
				// Close the connection
				let _ = tx.send(OwnedMessage::Close(None));
				break;
			}
			// Send a ping
			"/ping" => OwnedMessage::Ping(b"PING".to_vec()),
			// Otherwise, just send text
			_ => OwnedMessage::Text(trimmed.to_string()),
		};

		match tx.send(message) {
			Ok(()) => (),
			Err(e) => {
				println!("Main Loop: {:?}", e);
				break;
			}
		}
	}

	// We're exiting

	println!("Waiting for child threads to exit");

	let _ = send_loop.join();
	let _ = receive_loop.join();

	println!("Exited");
}