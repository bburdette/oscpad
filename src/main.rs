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
    let mapp = controls::makeControlMap(&*blah.rootControl);

    let q = String::new() + &guistring[..];

    let ipnport = String::new() + &ip[..] + ":3030";

    thread::spawn(move || { 
      winsockets_main(ip, q, mapp);
      });

    Iron::new(move |req: &mut Request| {
        let content_type = "text/html".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, &*htmlstring)))
    }).http(&ipnport[..]).unwrap();
}

type sendBlah = Arc<Mutex<sender::Sender<WebSocketStream>>>;


#[derive(Clone)]
struct Broadcaster {
    tvs : Arc<Mutex<Vec<sendBlah >>>
}

fn test (sa: &SocketAddr) -> bool { 
  match sa {
    &SocketAddr::V4(sav4) => true,
    _ => false,
  }
}

fn mysockeq(saleft: &SocketAddr, saright: &SocketAddr) -> bool {
  match (saleft, saright) {
    (&SocketAddr::V4(l),&SocketAddr::V4(r)) => l == r,
    (&SocketAddr::V6(l),&SocketAddr::V6(r)) => l == r,
    _ => false,
  }
}


impl Broadcaster {
    fn new() -> Broadcaster {
        Broadcaster {
            tvs : Arc::new(Mutex::new(Vec::new()))
        }
    }

    fn register(&mut self, sender : sendBlah) {
        let mut tvs = self.tvs.lock().unwrap();

        tvs.push(sender);
    }

    fn broadcast(&mut self, msg : Message) {
        let mut tvs = self.tvs.lock().unwrap();

        for tv in tvs.iter_mut() {
            let mut tvsend = tv.lock().unwrap();
            match tvsend.send_message(msg.clone()) {
                Err(e) => {},
                _ => {}
            }
        }
    }
   
    fn broadcast_others(&mut self, sa: &SocketAddr , msg : Message) {
      let mut tvs = self.tvs.lock().unwrap();

      for tv in tvs.iter_mut() {
        let mut tvsend = tv.lock().unwrap();
        match tvsend.get_mut().peer_addr() { 
          Ok(sa_send) => 
            if !mysockeq(sa,&sa_send) {
              match tvsend.send_message(msg.clone()) {
                Err(e) => {},
                _ => {}
              }
            },
          Err(e) => {},
        }
      }
    }
}


fn winsockets_main(ipaddr: String, aSConfig: String, cm: controls::controlMap ) {
  let ipnport = ipaddr + ":1234";
	let server = Server::bind(&ipnport[..]).unwrap();

  let shareblah = Arc::new(Mutex::new(cm));

	for connection in server {
		// Spawn a new thread for each connection.
    let blah = String::new() + &aSConfig[..];
    
    let scm = shareblah.clone();

    let mut broadcaster = Broadcaster::new();

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
			
			let (mut sender, mut receiver) = client.split();

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

