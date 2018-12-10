#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate websocket;

use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::thread;

use websocket::OwnedMessage;
use websocket::Message;
use websocket::sync::Server;
use websocket::ws::dataframe::DataFrame;

use std::sync::atomic::{AtomicPtr, Ordering as OtherOrdering};

// import from our own modules
mod sign_message;
mod request_info;
mod order_book;



// secret key of tesseract
const TESSERACT_SK_ETH: &str = "c5332bc1cd7584debcc5cb35884e8df8c550dd40c42fe5a31a20a33f985a4b0e";




fn main() {

    //server
	let server = Server::bind("127.0.0.1:2794").unwrap();

    let mut order_book_buy_btc_sell_eth = order_book::OrderBook {
    	buy_crypto_type: "BTC",
        sell_crypto_type: "ETH",
	    priority_queue: BinaryHeap::new()
    };

    let mut order_book_buy_eth_sell_btc = order_book::OrderBook {
    	buy_crypto_type: "ETH",
        sell_crypto_type: "BTC",
	    priority_queue: BinaryHeap::new()
    };


	for request in server.filter_map(Result::ok) {
        let mut atomic_ptr1 = AtomicPtr::new(&mut order_book_buy_btc_sell_eth);
        let mut atomic_ptr2 = AtomicPtr::new(&mut order_book_buy_eth_sell_btc);
        
		// Spawn a new thread for each connection.
		thread::spawn(move || {
			if !request.protocols().contains(&"rust-websocket".to_string()) {
				request.reject().unwrap();
				return;
			}

			let mut client = request.use_protocol("rust-websocket").accept().unwrap();

			let ip = client.peer_addr().unwrap();

			println!("Connection from {}", ip);

			let message = OwnedMessage::Text("Hello".to_string());
			client.send_message(&message).unwrap();

			let (mut receiver, _) = client.split().unwrap();

			for message in receiver.incoming_messages() {
                
				let message = message.unwrap();
                let message1 = websocket::Message::from(message);
                let mut message_vec = message1.take_payload();
                let mut atomic_ptr3 = AtomicPtr::new(&mut message_vec);
                unsafe {
                    let json_string = std::str::from_utf8(&*atomic_ptr3.load(OtherOrdering::Relaxed)).unwrap();
                    let mut request_info: request_info::RequestInfo = serde_json::from_str(&json_string).unwrap();
                    println!("{:#?}", request_info);
                    let mut atomic_ptr4 = AtomicPtr::new(&mut request_info);

                    println!("{} order: sell {} {}, buy {} {}", 
                                    (*atomic_ptr4.load(OtherOrdering::Relaxed)).user_id, (*atomic_ptr4.load(OtherOrdering::Relaxed)).sell_amount, (*atomic_ptr4.load(OtherOrdering::Relaxed)).sell_crypto_type, 
                                    (*atomic_ptr4.load(OtherOrdering::Relaxed)).buy_amount, (*atomic_ptr4.load(OtherOrdering::Relaxed)).buy_crypto_type);
                    
                    if request_info.request_type == "order" {
                            if request_info.sell_crypto_type == "ETH" {
                                (*atomic_ptr1.load(OtherOrdering::Relaxed)).priority_queue.push(&*atomic_ptr4.load(OtherOrdering::Relaxed));
                                println!("numbers of sell-ETH order: {}", (*atomic_ptr1.load(OtherOrdering::Relaxed)).priority_queue.len());
                                println!("top sell-ETH rate: {}", (*atomic_ptr1.load(OtherOrdering::Relaxed)).priority_queue.peek().unwrap().rate);
                            } else {
                                (*atomic_ptr2.load(OtherOrdering::Relaxed)).priority_queue.push(&*atomic_ptr4.load(OtherOrdering::Relaxed));
                                println!("numbers of buy-ETH order: {}", (*atomic_ptr2.load(OtherOrdering::Relaxed)).priority_queue.len());
                                println!("top buy-ETH rate: {}", (*atomic_ptr2.load(OtherOrdering::Relaxed)).priority_queue.peek().unwrap().rate);
                            }
                            if (*atomic_ptr1.load(OtherOrdering::Relaxed)).priority_queue.len() != 0 && (*atomic_ptr2.load(OtherOrdering::Relaxed)).priority_queue.len() != 0 {
                                if (*atomic_ptr1.load(OtherOrdering::Relaxed)).priority_queue.peek().unwrap().rate ==  (*atomic_ptr2.load(OtherOrdering::Relaxed)).priority_queue.peek().unwrap().rate {
                                    println!("MATCHED!!! {} and {} matched at rate = {}", (*atomic_ptr1.load(OtherOrdering::Relaxed)).priority_queue.peek().unwrap().user_id, (*atomic_ptr2.load(OtherOrdering::Relaxed)).priority_queue.peek().unwrap().user_id, (*atomic_ptr1.load(OtherOrdering::Relaxed)).priority_queue.peek().unwrap().rate);
                                    (*atomic_ptr1.load(OtherOrdering::Relaxed)).priority_queue.pop().unwrap().rate;
                                    (*atomic_ptr2.load(OtherOrdering::Relaxed)).priority_queue.pop().unwrap().rate;
                                    println!("numbers of sell-ETH order: {}", (*atomic_ptr1.load(OtherOrdering::Relaxed)).priority_queue.len());
                                    println!("numbers of buy-ETH order: {}", (*atomic_ptr2.load(OtherOrdering::Relaxed)).priority_queue.len());
                                }
                            }          
                    } else {
                        let message_to_sign: String = (*atomic_ptr4.load(OtherOrdering::Relaxed)).address.to_string() + &(*atomic_ptr4.load(OtherOrdering::Relaxed)).coin_id.to_string() + &(*atomic_ptr4.load(OtherOrdering::Relaxed)).nonce.to_string();
                        sign_message::sign_message(message_to_sign, TESSERACT_SK_ETH.to_string());
                    }
                }
			}
		}); 
	}
}
