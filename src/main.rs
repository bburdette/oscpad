#![feature(custom_derive)]

extern crate websocket;


use std::thread;
use std::sync::{Arc, Mutex};
use std::fmt::format;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::os;
use std::env;
use std::io::Read;
use std::io::{Error,ErrorKind};
use std::string::*;
use std::collections::BTreeMap;

use std::net::UdpSocket;
use std::net::SocketAddr;

use websocket::{Server, Message, Sender, Receiver};
use websocket::header::WebSocketProtocol;
use websocket::stream::WebSocketStream;
use websocket::server::sender;

extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

mod controls;
mod broadcaster;

extern crate serde_json;
use serde_json::Value;

fn loadString(file_name: &str) -> Option<String>
{
  let path = &Path::new(&file_name);
  let f = File::open(path);
  let mut result = String::new();
  match f { 
    Ok(mut of) => {
      match of.read_to_string(&mut result) { 
        Err(e) => {
          println!("err: {:?}", e);
          None
        },
        Ok(len) => { 
          println!("read {} bytes", len);
          Some(result)
        }, 
      } 
    },
    Err(e) => {
      println!("err: {:?}", e);
      None
    },
  }
}


fn main() {
  // read in the settings json.
  let args = env::args();
  let mut iter = args.skip(1); // skip the program name
  match iter.next() {
    Some(file_name) => {
      
      println!("loading config file: {}", file_name);
      let config = loadString(&file_name).unwrap();
      
      // read config file as json
      let data: Value = serde_json::from_str(&config[..]).unwrap();

      startserver(data)

    }
    None => {
      println!("no args!");
    }
  }
}

fn startserver(config: Value) 
{
    let obj = config.as_object().unwrap();
    
    let htmlfilename = 
      obj.get("htmlfile").unwrap().as_string().unwrap();
    let htmlstring = loadString(&htmlfilename[..]).unwrap();

    let guifilename = 
      obj.get("guifile").unwrap().as_string().unwrap();
    let guistring = loadString(&guifilename[..]).unwrap();

    println!("config: {:?}", config);

    let ip = String::new() + 
      obj.get("ip").unwrap().as_string().unwrap();

    let wip = String::new() + 
      obj.get("wip").unwrap().as_string().unwrap();

    // deserialize the gui string into json.
    let guival: Value = serde_json::from_str(&guistring[..]).unwrap();
    let blah = controls::deserializeRoot(&guival).unwrap();

    println!("title: {} count: {} ", 
      blah.title, blah.rootControl.controlType());
    println!("controls: {:?}", blah.rootControl);

    // from control tree, make a map of ids->controls.
    let mapp = controls::makeControlMap(&*blah.rootControl);
    let cm = Arc::new(Mutex::new(mapp));

    let q = String::new() + &guistring[..];

    let ipnport = String::new() + &ip[..] + ":3030";

    let mut broadcaster = broadcaster::Broadcaster::new();
    
    thread::spawn(move || { 
      winsockets_main(wip, q, cm, broadcaster);
      });

    Iron::new(move |req: &mut Request| {
        let content_type = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, &*htmlstring)))
    }).http(&ipnport[..]).unwrap();
}

// fn winsockets_main(ipaddr: String, aSConfig: String, cm: controls::controlMap ) {
fn winsockets_main( ipaddr: String, 
                    aSConfig: String, 
                    cm: Arc<Mutex<controls::controlMap>>,
                    broadcaster: broadcaster::Broadcaster)
{
  // let ipnport = ipaddr + ":1234";
	let server = Server::bind(&ipaddr[..]).unwrap();

    

	for connection in server {
		// Spawn a new thread for each connection.
    let blah = String::new() + &aSConfig[..];
    
    let scm = cm.clone();

    let mut broadcaster = broadcaster.clone();

    thread::spawn(move || {
      // Get the request
			let request = connection.unwrap().read_request().unwrap(); 
      // Keep the headers so we can check them
			let headers = request.headers.clone(); 
			
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
     
      let message = Message::Text(blah);
			client.send_message(message.clone()).unwrap();
			
			let (sender, mut receiver) = client.split();

      let sendmeh = Arc::new(Mutex::new(sender));
      
      broadcaster.register(sendmeh.clone());      
			
			for message in receiver.incoming_messages() {
				let message = message.unwrap();
        println!("message: {:?}", message);
	
        let replyy = Message::Text("replyyyblah".to_string());
	
				match message {
					Message::Close(_) => {
						let message = Message::Close(None);
            let mut sender = sendmeh.lock().unwrap();
						sender.send_message(message).unwrap();
						println!("Client {} disconnected", ip);
						return;
					}
					Message::Ping(data) => {
            println!("Message::Ping(data)");
						let message = Message::Pong(data);
            let mut sender = sendmeh.lock().unwrap();
						sender.send_message(message).unwrap();
					}
          Message::Text(texxt) => {
            let jsonval: Value = serde_json::from_str(&texxt[..]).unwrap();
            let s_um = controls::decodeUpdateMessage(&jsonval);
            match s_um { 
              Some(updmsg) => {
                let mut scmun = scm.lock().unwrap();
                let cntrl = scmun.get_mut(controls::getUmId(&updmsg));
                match cntrl {
                  Some(x) => {
                    (*x).update(&updmsg);
                    println!("some x: {:?}", *x);
                    broadcaster.broadcast_others(&ip, Message::Text(texxt))
                  },
                  None => println!("none"),
                }
              },
              _ => println!("uknown msg"),
            }
          },
					_ => { 
            println!("sending replyy");
            let mut sender = sendmeh.lock().unwrap();
            sender.send_message(replyy).unwrap()
          }
				}
			}
		});
	}
}

