#![feature(custom_derive)]

extern crate websocket;

use std::thread;
use std::sync::{Arc, Mutex};
use std::fs;
use std::fs::File;
use std::path::Path;
use std::os;
use std::env;
use std::io::Read;
use std::string::*;
use std::collections::BTreeMap;

use websocket::{Server, Message, Sender, Receiver};
use websocket::header::WebSocketProtocol;

extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;

mod controls;

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

    // going to serve up html 
    let mut htmlstring = String::new();
    // on connect, send up json
    let mut guistring = String::new();

    // ip address.  I guess!
    let mut ip = String::new();

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

    //println!("postsereialsis");
    // let meh = String::new() + blah.rootControl.controlType();  

    let ip = String::new() + 
      obj.get("ip").unwrap().as_string().unwrap();

    // deserialize the gui string into json.
    let guival: Value = serde_json::from_str(&guistring[..]).unwrap();
    let blah = controls::deserializeRoot(&guival).unwrap();

    println!("title: {} count: {} ", 
      blah.title, blah.rootControl.controlType());
    println!("controls: {:?}", blah.rootControl);

    // from control tree, make a map of ids->controls.
    let sharemap = 
      Arc::new(
        Mutex::new(
          controls::makeControlMap(&*blah.rootControl)));  

    println!("\n\nsharemap: {:?}", sharemap);

    let q = String::new() + &guistring[..];

    let ipnport = String::new() + &ip[..] + ":3030";

    thread::spawn(move || { 
      winsockets_main(ip, q, sharemap);
      });

    Iron::new(move |req: &mut Request| {
        let content_type = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, &*htmlstring)))
    }).http(&ipnport[..]).unwrap();
}

type sharedControlMap = Arc< Mutex< controls::controlMap > >;

fn winsockets_main(ipaddr: String, aSConfig: String, scm: sharedControlMap ) {
  let ipnport = ipaddr + ":1234";
	let server = Server::bind(&ipnport[..]).unwrap();

	for connection in server {
		// Spawn a new thread for each connection.
    let blah = String::new() + &aSConfig[..];
		
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
     
      let message = Message::Text(blah);
			client.send_message(message.clone()).unwrap();
			
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
          Message::Text(texxt) => {
            let jsonval: Value = serde_json::from_str(&texxt[..]).unwrap();
            let s_um = controls::decodeUpdateMessage(&jsonval);
            match s_um { 
              Some(controls::UpdateMsg::Button{ controlId: cid, updateType: ut }) => {
                println!("button msg {:?} ", cid)
                },
              Some(controls::UpdateMsg::Slider{ controlId: cid, updateType: ut, location: l}) => {
                println!("slider msg {:?} location: {}", cid, l)
                },
              _ => println!("uknown msg"),
              }
            },
					_ => { 
            println!("sending replyy");
            sender.send_message(replyy).unwrap()
            }
				}
			}
		});
	}
}

