extern crate websocket;

use std::thread;
use websocket::{Server, Message, Sender, Receiver};
use websocket::header::WebSocketProtocol;

extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::os;
use std::env;
use std::io::Read;
use std::string::*;

extern crate serde_json;
use serde_json::Value;

fn main() {

    let args = env::args();
    let mut iter = args.skip(1); // skip the program name

    let mut s = String::new();

    match iter.next() {
      Some(file_name) => {
        println!("{}", file_name);
        let path = &Path::new(&file_name);
        let f = File::open(path);
        match f { 
          Ok(mut of) => {
            match of.read_to_string(&mut s) { 
              Err(e) => {
                println!("err: {:?}", e);
                return ();
                },
              Ok(len) => { 
                println!("read {} bytes", len)
                }, 
            } 
            },
          Err(e) => {
            println!("err: {:?}", e);
            return ();
            },
          }
        println!("serving file: {}", file_name);
        println!("from localhost:3000");
  
        let data: Value = serde_json::from_str(&s[..]).unwrap();

        println!("serdeval: {:?}", data);

        }
      None => {
        println!("no args!");
        }
    }

    thread::spawn(move || { 
      winsockets_main();
      });

    Iron::new(move |req: &mut Request| {
        let content_type = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, &*s)))
    }).http("localhost:3000").unwrap();
}


fn winsockets_main() {
	let server = Server::bind("127.0.0.1:1234").unwrap();

	for connection in server {
		// Spawn a new thread for each connection.
		thread::spawn(move || {
			let request = connection.unwrap().read_request().unwrap(); // Get the request
			let headers = request.headers.clone(); // Keep the headers so we can check them
			
			request.validate().unwrap(); // Validate the request
			
			let mut response = request.accept(); // Form a response
			
			if let Some(&WebSocketProtocol(ref protocols)) = headers.get() {
				if protocols.contains(&("rust-websocket".to_string())) {
					// We have a protocol we want to use
					response.headers.set(WebSocketProtocol(vec!["rust-websocket".to_string()]));
				}
			}
			
			let mut client = response.send().unwrap(); // Send the response
			
			let ip = client.get_mut_sender()
				.get_mut()
				.peer_addr()
				.unwrap();
			
			println!("Connection from {}", ip);
			
			let message = Message::Text("Hello".to_string());
			client.send_message(message).unwrap();
			
			let (mut sender, mut receiver) = client.split();
			
			for message in receiver.incoming_messages() {
				let message = message.unwrap();
        println!("message: {:?}", message);
			
        let replyy = Message::Text("replyyyblah".to_string());
	
				match message {
					Message::Close(_) => {
						let message = Message::Close(None);
						sender.send_message(message).unwrap();
						println!("Client {} disconnected", ip);
						return;
					}
					Message::Ping(data) => {
            println!("Message::Ping(data)");
						let message = Message::Pong(data);
						sender.send_message(message).unwrap();
					}
					// _ => sender.send_message(message).unwrap(),
					_ => { 
            println!("sending replyy");
            sender.send_message(replyy).unwrap()
            }
				}
			}
		});
	}
}

